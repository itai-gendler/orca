//! Folder review state stays in the UI; persistence and scanning stay in services.
use crate::{protocol::Request, AppState, FolderChoice, FolderRuleDraft, OrcaWindow};
use orca_services::FolderNode;
use slint::{ComponentHandle, Model, VecModel};
use std::{path::Path, rc::Rc, sync::mpsc::Sender};

pub(crate) fn drafts(state: &AppState) -> Vec<FolderRuleDraft> {
    let model = state.get_folder_rules();
    (0..model.row_count())
        .filter_map(|i| model.row_data(i))
        .collect()
}

pub(crate) fn choices(nodes: Vec<FolderNode>, rules: &[FolderRuleDraft]) -> Vec<FolderChoice> {
    nodes
        .into_iter()
        .map(|node| {
            let rule = rules
                .iter()
                .filter(|rule| Path::new(node.path.as_str()).starts_with(rule.path.as_str()))
                .max_by_key(|rule| rule.path.len());
            let mode = rules
                .iter()
                .find(|rule| rule.path == node.path)
                .map_or(node.explicit.as_str(), |rule| rule.mode.as_str());
            FolderChoice {
                path: node.path.into(),
                name: node.name.into(),
                mode: mode.into(),
                included: rule.map_or(node.included, |rule| rule.mode == "include"),
                has_children: node.has_children,
            }
        })
        .collect()
}

pub(crate) fn install(ui: &OrcaWindow, tx: Sender<Request>) {
    let weak = ui.as_weak();
    ui.global::<AppState>()
        .on_folder_tree(move |action, path, mode| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let state = ui.global::<AppState>();
            let root = state.get_folder_root().to_string();
            let current = state.get_folder_current().to_string();
            let adding = state.get_folder_adding();
            match action.as_str() {
                "cancel" => state.set_folder_dialog(false),
                "finish" => {
                    if adding {
                        let rules = drafts(&state)
                            .into_iter()
                            .map(|rule| (rule.path.to_string(), rule.mode.to_string()))
                            .collect();
                        let _ = tx.send(Request::AddFolder(root, rules));
                    }
                    state.set_folder_dialog(false);
                }
                "open" if Path::new(path.as_str()).starts_with(&root) && path != current => {
                    let _ = tx.send(Request::FolderPreview {
                        root,
                        current: path.into(),
                        adding,
                    });
                }
                "back" => {
                    let parent = Path::new(&current).parent().unwrap_or(Path::new(&root));
                    let parent = if parent.starts_with(&root) {
                        parent
                    } else {
                        Path::new(&root)
                    };
                    let current = parent.to_string_lossy().into_owned();
                    let _ = tx.send(Request::FolderPreview {
                        root,
                        current,
                        adding,
                    });
                }
                "rule" if matches!(mode.as_str(), "include" | "exclude" | "inherit") => {
                    if adding {
                        let mut rules = drafts(&state);
                        rules.retain(|rule| rule.path != path);
                        if mode != "inherit" {
                            rules.push(FolderRuleDraft {
                                path: path.clone(),
                                mode: mode.clone(),
                            });
                        }
                        state.set_folder_rules(Rc::new(VecModel::from(rules.clone())).into());
                        let model = state.get_folder_choices();
                        let mut rows = (0..model.row_count())
                            .filter_map(|i| model.row_data(i))
                            .collect::<Vec<_>>();
                        for row in &mut rows {
                            let rule = rules
                                .iter()
                                .filter(|rule| {
                                    Path::new(row.path.as_str()).starts_with(rule.path.as_str())
                                })
                                .max_by_key(|rule| rule.path.len());
                            if row.path == path {
                                row.mode = mode.clone();
                            }
                            row.included = rule.is_none_or(|rule| rule.mode == "include");
                        }
                        state.set_folder_choices(Rc::new(VecModel::from(rows)).into());
                    } else {
                        let _ = tx.send(Request::SetFolderRule {
                            root,
                            path: path.into(),
                            mode: mode.into(),
                        });
                    }
                }
                _ => {}
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_choices_inherit_parent_and_allow_nested_exceptions() {
        let rules = vec![
            FolderRuleDraft {
                path: "C:/Music/Skip".into(),
                mode: "exclude".into(),
            },
            FolderRuleDraft {
                path: "C:/Music/Skip/Keep".into(),
                mode: "include".into(),
            },
        ];
        let nodes = [
            "C:/Music/Skip/New",
            "C:/Music/Skip/Keep/New",
            "C:/Music/Skip-extra",
        ]
        .into_iter()
        .map(|path| FolderNode {
            path: path.into(),
            name: path.into(),
            explicit: "inherit".into(),
            included: true,
            has_children: false,
        })
        .collect();
        let rows = choices(nodes, &rules);
        assert!(!rows[0].included);
        assert!(rows[1].included);
        assert!(rows[2].included);
    }
}
