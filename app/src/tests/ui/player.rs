use super::*;

pub(super) fn verify(case: &mut UiCase) {
    let ui = &case.ui;
    let window = &case.window;
    let state = ui.global::<AppState>();
    let pixels = &mut case.pixels;
    // Hovering or dragging the popup must keep it open past its close delay.
    window.dispatch_event(WindowEvent::ScaleFactorChanged { scale_factor: 1.0 });
    window.set_size(slint::PhysicalSize::new(1280, 820));
    state.set_blurred_background(false);
    let previous_song = state.get_now();
    state.set_full_player(false);
    state.set_lyrics_open(false);
    state.set_queue_open(false);
    state.set_view("songs".into());
    state.set_now(Song {
        id: 42,
        path: "playing.flac".into(),
        title: "Playing song".into(),
        ..Default::default()
    });
    let (playlist_tx, playlist_rx) = std::sync::mpsc::channel();
    controllers::playlists::install(ui, playlist_tx);
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    screenshot(pixels, "player-controls-smoke");
    click(window, 1104.0, 772.0);
    assert!(state.get_playlist_picker_open());
    assert!(matches!(
        playlist_rx.try_recv().unwrap(),
        Request::PlaylistChoices(42)
    ));
    state.set_playlist_choices(
        Rc::new(VecModel::from(vec![
            PlaylistChoice {
                key: "7".into(),
                title: "Favorites".into(),
                selected: false,
            },
            PlaylistChoice {
                key: "8".into(),
                title: "Road trip".into(),
                selected: true,
            },
        ]))
        .into(),
    );
    state.set_playlist_picker_loading(false);
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    screenshot(pixels, "playlist-picker-toggle");
    click(window, 1100.0, 584.0);
    assert!(matches!(
        playlist_rx.try_recv().unwrap(),
        Request::Playlist(crate::protocol::PlaylistEdit::Add { id: 7, song_id: 42 })
    ));
    assert!(matches!(
        playlist_rx.try_recv().unwrap(),
        Request::PlaylistChoices(42)
    ));
    assert!(state.get_playlist_picker_open());
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    click(window, 1100.0, 634.0);
    assert!(matches!(
        playlist_rx.try_recv().unwrap(),
        Request::Playlist(crate::protocol::PlaylistEdit::Remove { id: 8, song_id: 42 })
    ));
    assert!(matches!(
        playlist_rx.try_recv().unwrap(),
        Request::PlaylistChoices(42)
    ));
    click(window, 800.0, 400.0);
    assert!(!state.get_playlist_picker_open());
    click(window, 1152.0, 772.0);
    assert!(state.get_lyrics_open());
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    screenshot(pixels, "inline-lyrics-docked");
    assert!(state.get_lyrics_width() > 200.0);
    click(window, 1232.0, 48.0);
    assert!(!state.get_lyrics_open());
    window.set_size(slint::PhysicalSize::new(560, 820));
    let mut narrow_pixels = vec![slint::Rgb8Pixel::default(); 560 * 820];
    window.draw_if_needed(|renderer| {
        renderer.render(&mut narrow_pixels, 560);
    });
    let bytes: Vec<u8> = narrow_pixels.iter().flat_map(|p| [p.r, p.g, p.b]).collect();
    save_screenshot(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target/slint/test-artifacts/ui/player-controls-narrow.png"),
        &bytes,
        560,
        820,
        image::ColorType::Rgb8,
    )
    .unwrap();
    click(window, 420.0, 698.0);
    assert!(state.get_playlist_picker_open());
    assert!(matches!(
        playlist_rx.try_recv().unwrap(),
        Request::PlaylistChoices(42)
    ));
    click(window, 100.0, 500.0);
    assert!(!state.get_playlist_picker_open());
    click(window, 459.0, 698.0);
    assert!(state.get_lyrics_open());
    click(window, 512.0, 48.0);
    assert!(!state.get_lyrics_open());
    window.set_size(slint::PhysicalSize::new(1280, 820));
    state.set_playlists(Default::default());
    state.set_now(previous_song);
    state.set_volume(0.5);
    let weak_volume = ui.as_weak();
    let volume_changes = Rc::new(Cell::new(0));
    let observed_volume = volume_changes.clone();
    state.on_volume_changed(move |value| {
        observed_volume.set(observed_volume.get() + 1);
        if let Some(ui) = weak_volume.upgrade() {
            ui.global::<AppState>().set_volume(value);
        }
    });
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    window.dispatch_event(WindowEvent::PointerMoved {
        position: slint::LogicalPosition::new(1244.0, 764.0),
    });
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    window.dispatch_event(WindowEvent::PointerMoved {
        position: slint::LogicalPosition::new(1244.0, 666.0),
    });
    std::thread::sleep(Duration::from_millis(650));
    slint::platform::update_timers_and_animations();
    click(window, 1244.0, 622.0);
    assert!(
        state.get_volume() > 0.8,
        "volume popup must remain interactive while hovered"
    );
    window.dispatch_event(WindowEvent::PointerPressed {
        position: slint::LogicalPosition::new(1244.0, 666.0),
        button: PointerEventButton::Left,
    });
    window.dispatch_event(WindowEvent::PointerMoved {
        position: slint::LogicalPosition::new(1244.0, 712.0),
    });
    assert!(
        state.get_volume() < 0.4,
        "dragging the popup must adjust volume"
    );
    std::thread::sleep(Duration::from_millis(650));
    slint::platform::update_timers_and_animations();
    window.dispatch_event(WindowEvent::PointerReleased {
        position: slint::LogicalPosition::new(1244.0, 712.0),
        button: PointerEventButton::Left,
    });
    assert!(volume_changes.get() >= 3);
    state.set_tray_available(false);
    set_background_playback(ui, true);
    assert!(
        !state.get_window_hidden(),
        "hiding is unavailable without a recovery tray"
    );
    state.set_tray_available(true);
    let was_full = state.get_full_player();
    set_background_playback(ui, true);
    assert!(state.get_window_hidden());
    assert_eq!(
        state.get_full_player(),
        was_full,
        "Phantom must preserve the current page"
    );
    let (tray_tx, tray_rx) = std::sync::mpsc::channel();
    for action in ["toggle", "previous", "next"] {
        dispatch_tray_action(ui, &tray_tx, action);
        assert!(
            matches!(tray_rx.try_recv().unwrap(),Request::Command(actual) if actual==crate::protocol::PlayerAction::parse(action).unwrap())
        );
    }
    dispatch_tray_action(ui, &tray_tx, "restore");
    assert!(!state.get_window_hidden());
    assert_eq!(state.get_full_player(), was_full);
    toggle_background_playback(ui);
    assert!(state.get_window_hidden());
    toggle_background_playback(ui);
    assert!(!state.get_window_hidden());
    assert_eq!(state.get_full_player(), was_full);
    state.set_now(Song {
        title: "Stale song".into(),
        artist: "Stale artist".into(),
        album: "Stale album".into(),
        path: "removed.wav".into(),
        ..Default::default()
    });
    state.set_position(12000.0);
    state.set_duration(20000.0);
    state.set_playing(true);
    state.set_collection_dialog(true);
    let original = serde_json::json!({"action":"collection-edit","kind":"artists","key":"Artist","secondary":"","draft_id":"1"});
    state.set_collection_request(original.to_string().into());
    let mut staged = original.clone();
    staged["cover"] = serde_json::json!("");
    assert!(stage_collection_draft(ui, &staged));
    assert!(state.get_collection_cover_missing());
    state.set_full_player(false);
    state.set_collection_kind("artists".into());
    state.set_collection_name("Preview artist".into());
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    screenshot(pixels, "artist-editor-preview-smoke");
    state.set_collection_kind("albums".into());
    state.set_collection_name("Preview album".into());
    state.set_scale(1.2);
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    screenshot(pixels, "album-editor-preview-smoke");
    state.set_scale(1.0);
    staged["draft_id"] = serde_json::json!("old-editor");
    assert!(!stage_collection_draft(ui, &staged));
    state.set_collection_dialog(false);
    assert!(!stage_collection_draft(ui, &original));
    state.set_full_player(false);
    state.set_detail_key("".into());
    state.set_view("artists".into());
    state.set_browse_artwork(true);
    let group = CatalogGroup {
        key: "Right-click artist".into(),
        title: "Right-click artist".into(),
        cover_missing: true,
        ..Default::default()
    };
    state.set_group_grid(
        Rc::new(VecModel::from(vec![
            Rc::new(VecModel::from(vec![group])).into()
        ]))
        .into(),
    );
    let selected = Rc::new(RefCell::new(String::new()));
    let observed = selected.clone();
    state.on_collection_menu(move |group, _, _| *observed.borrow_mut() = group.key.to_string());
    window.draw_if_needed(|renderer| {
        renderer.render(pixels, 1280);
    });
    state.set_keyboard_navigation(true);
    window.dispatch_event(WindowEvent::PointerPressed {
        position: slint::LogicalPosition::new(125.0, 130.0),
        button: PointerEventButton::Right,
    });
    window.dispatch_event(WindowEvent::PointerReleased {
        position: slint::LogicalPosition::new(125.0, 130.0),
        button: PointerEventButton::Right,
    });
    assert_eq!(selected.borrow().as_str(), "Right-click artist");
    assert!(
        !state.get_keyboard_navigation(),
        "right-click menus must hide keyboard-only focus outlines"
    );
    reset_empty_player(ui);
    assert!(state.get_now().path.is_empty());
    assert!(state.get_now().title.is_empty());
    assert!(state.get_now().artist.is_empty());
    assert!(state.get_now().album.is_empty());
    assert!(state.get_now().cover_missing);
    assert_eq!(state.get_position(), 0.0);
    assert_eq!(state.get_duration(), 0.0);
    assert!(!state.get_playing());
    assert!(state.get_original_cover_missing());
}
