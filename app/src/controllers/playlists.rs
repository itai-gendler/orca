//! Playlist editor/import/export callbacks; service owns database changes.
use crate::{
    protocol::{PlaylistEdit, Request},
    AppState, OrcaWindow,
};
use orca_services::operation_types::OperationRequest;
use slint::{ComponentHandle, Model};
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
    sync::mpsc::Sender,
};
pub(crate) fn install(ui: &OrcaWindow, tx: Sender<Request>) {
    let state = ui.global::<AppState>();
    let selected = Rc::new(RefCell::new(HashSet::<i32>::new()));
    let selection_anchor = Rc::new(Cell::new(None::<i32>));
    let drag_base = Rc::new(RefCell::new(HashSet::<i32>::new()));
    {
        let selected = selected.clone();
        state.on_playlist_song_selected(move |song_id, _revision| {
            selected.borrow().contains(&song_id)
        });
    }
    {
        let selected = selected.clone();
        let selection_anchor = selection_anchor.clone();
        let drag_base = drag_base.clone();
        let weak = ui.as_weak();
        let tx = tx.clone();
        state.on_playlist_selection(move |action, song_id| {
            let Some(ui) = weak.upgrade() else { return };
            let state = ui.global::<AppState>();
            match action.as_str() {
                "start"
                    if state.get_view() == "playlists" && !state.get_detail_key().is_empty() =>
                {
                    selected.borrow_mut().clear();
                    drag_base.borrow_mut().clear();
                    selection_anchor.set(None);
                    state.set_playlist_selection_count(0);
                    state.set_playlist_selection_mode(true);
                }
                "toggle" if state.get_playlist_selection_mode() && song_id > 0 => {
                    let count = {
                        let mut chosen = selected.borrow_mut();
                        if !chosen.insert(song_id) {
                            chosen.remove(&song_id);
                        }
                        chosen.len() as i32
                    };
                    state.set_playlist_selection_count(count);
                }
                "anchor" if state.get_playlist_selection_mode() && song_id >= 0 => {
                    selection_anchor.set(Some(song_id));
                }
                "range" | "drag-to" if state.get_playlist_selection_mode() && song_id >= 0 => {
                    let songs = state.get_songs();
                    let anchor = selection_anchor.get().unwrap_or(song_id);
                    let upper = songs.row_count().saturating_sub(1) as i32;
                    let start = anchor.min(song_id).clamp(0, upper);
                    let end = anchor.max(song_id).clamp(0, upper);
                    let mut chosen = if action == "drag-to" {
                        drag_base.borrow().clone()
                    } else {
                        selected.borrow().clone()
                    };
                    for index in start..=end {
                        if let Some(song) = songs.row_data(index as usize) {
                            if song.id > 0 {
                                chosen.insert(song.id);
                            }
                        }
                    }
                    let count = chosen.len() as i32;
                    *selected.borrow_mut() = chosen;
                    state.set_playlist_selection_count(count);
                }
                "drag-start" if state.get_playlist_selection_mode() && song_id >= 0 => {
                    selection_anchor.set(Some(song_id));
                    *drag_base.borrow_mut() = selected.borrow().clone();
                }
                "remove" if state.get_playlist_selection_mode() => {
                    let song_ids: Vec<i64> =
                        selected.borrow().iter().map(|id| *id as i64).collect();
                    if !song_ids.is_empty() {
                        let id = if state.get_detail_key() == "liked" {
                            None
                        } else {
                            match state.get_detail_key().parse::<i64>() {
                                Ok(id) if id > 0 => Some(id),
                                _ => return,
                            }
                        };
                        let _ =
                            tx.send(Request::Playlist(PlaylistEdit::RemoveMany { id, song_ids }));
                    }
                    selected.borrow_mut().clear();
                    drag_base.borrow_mut().clear();
                    selection_anchor.set(None);
                    state.set_playlist_selection_count(0);
                    state.set_playlist_selection_mode(false);
                }
                "cancel" => {
                    selected.borrow_mut().clear();
                    drag_base.borrow_mut().clear();
                    selection_anchor.set(None);
                    state.set_playlist_selection_count(0);
                    state.set_playlist_selection_mode(false);
                }
                _ => return,
            }
            state.set_playlist_selection_revision(
                state.get_playlist_selection_revision().wrapping_add(1),
            );
        });
    }
    {
        let weak = ui.as_weak();
        let tx = tx.clone();
        state.on_toggle_like(move || {
            if let Some(ui) = weak.upgrade() {
                let state = ui.global::<AppState>();
                let song_id = state.get_now().id;
                if song_id > 0 {
                    let _ = tx.send(Request::ToggleLiked(song_id as i64));
                }
            }
        });
    }
    {
        let tx = tx.clone();
        let weak = ui.as_weak();
        state.on_playlist_action(move |action, value| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let s = ui.global::<AppState>();
            match action.as_str() {
                "import" | "export" => {
                    let tx = tx.clone();
                    let export = action == "export";
                    let id = s.get_detail_key().parse::<i64>().unwrap_or(0);
                    std::thread::spawn(move || {
                        let dialog =
                            rfd::FileDialog::new().add_filter("Playlists", &["m3u", "m3u8"]);
                        let path = if export {
                            dialog.set_file_name("playlist.m3u8").save_file()
                        } else {
                            dialog.pick_file()
                        };
                        if let Some(path) = path {
                            let file = path.to_string_lossy().into_owned();
                            let request = if export {
                                OperationRequest::ExportPlaylist { id, file }
                            } else {
                                OperationRequest::ImportPlaylist { file }
                            };
                            let _ = tx.send(Request::Operation(request));
                        }
                    });
                }
                "new-dialog" | "rename-dialog" => {
                    let rename = action == "rename-dialog";
                    s.set_playlist_edit_action(if rename { "rename" } else { "create" }.into());
                    s.set_playlist_name(if rename {
                        s.get_detail_title()
                    } else {
                        "".into()
                    });
                    s.set_playlist_dialog(true);
                }
                "open-current" => {
                    let song_id = s.get_now().id;
                    if song_id > 0 {
                        s.set_playlist_picker_loading(true);
                        s.set_playlist_choices(Default::default());
                        s.set_playlist_picker_open(true);
                        let _ = tx.send(Request::PlaylistChoices(song_id as i64));
                    }
                }
                "create" | "rename" => {
                    let edit = if action == "create" {
                        PlaylistEdit::Create { name: value.into() }
                    } else {
                        PlaylistEdit::Rename {
                            id: s.get_detail_key().parse().unwrap_or(0),
                            name: value.into(),
                        }
                    };
                    let _ = tx.send(Request::Playlist(edit));
                    if action == "create" && s.get_playlist_picker_open() && s.get_now().id > 0 {
                        s.set_playlist_picker_loading(true);
                        let _ = tx.send(Request::PlaylistChoices(s.get_now().id as i64));
                    }
                }
                "delete" => {
                    let _ = tx.send(Request::Playlist(PlaylistEdit::Delete {
                        id: value.parse().unwrap_or(0),
                    }));
                }
                "remove-menu" => {
                    let song_id = s.get_menu_song().id;
                    if song_id <= 0 || s.get_view() != "playlists" || s.get_detail_key().is_empty()
                    {
                        return;
                    }
                    let id = if s.get_detail_key() == "liked" {
                        None
                    } else {
                        match s.get_detail_key().parse::<i64>() {
                            Ok(id) if id > 0 => Some(id),
                            _ => return,
                        }
                    };
                    let _ = tx.send(Request::Playlist(PlaylistEdit::RemoveMany {
                        id,
                        song_ids: vec![song_id as i64],
                    }));
                }
                "add" | "add-current" | "remove-current" => {
                    let song_id = if action == "add-current" || action == "remove-current" {
                        s.get_now().id
                    } else {
                        s.get_menu_song().id
                    };
                    if song_id <= 0 {
                        return;
                    }
                    let id = value.parse().unwrap_or(0);
                    let edit = if action == "remove-current" {
                        PlaylistEdit::Remove {
                            id,
                            song_id: song_id as i64,
                        }
                    } else {
                        PlaylistEdit::Add {
                            id,
                            song_id: song_id as i64,
                        }
                    };
                    let _ = tx.send(Request::Playlist(edit));
                    if action != "add" {
                        s.set_playlist_picker_loading(true);
                        let _ = tx.send(Request::PlaylistChoices(song_id as i64));
                    }
                }
                _ => {}
            }
        });
    }
}
