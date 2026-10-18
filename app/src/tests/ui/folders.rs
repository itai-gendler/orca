use super::*;

pub(super) fn verify(case: &mut UiCase) {
    let state = case.ui.global::<AppState>();
    let (tx, rx) = std::sync::mpsc::channel();
    controllers::folders::install(&case.ui, tx);
    state.set_folder_root("C:/Music".into());
    state.set_folder_current("C:/Music".into());
    state.set_folder_adding(true);
    state.set_folder_dialog(true);
    state.set_folder_choices(
        Rc::new(VecModel::from(vec![
            FolderChoice {
                path: "C:/Music/Skip".into(),
                name: "Skip".into(),
                mode: "inherit".into(),
                included: true,
                has_children: true,
            },
            FolderChoice {
                path: "C:/Music/Albums".into(),
                name: "Albums".into(),
                mode: "include".into(),
                included: true,
                has_children: true,
            },
            FolderChoice {
                path: "C:/Music/Mixes".into(),
                name: "Mixes".into(),
                mode: "exclude".into(),
                included: false,
                has_children: false,
            },
        ]))
        .into(),
    );
    case.window.set_size(slint::PhysicalSize::new(1280, 820));
    case.window.draw_if_needed(|renderer| {
        renderer.render(&mut case.pixels, 1280);
    });
    screenshot(&case.pixels, "folder-review-dialog");
    click(&case.window, 896.0, 400.0);
    assert!(!state.get_folder_choices().row_data(0).unwrap().included);
    case.window.draw_if_needed(|renderer| {
        renderer.render(&mut case.pixels, 1280);
    });
    click(&case.window, 820.0, 400.0);
    let inherited = state.get_folder_choices().row_data(0).unwrap();
    assert_eq!(inherited.mode, "inherit");
    assert!(inherited.included);
    state.invoke_folder_tree("rule".into(), "C:/Music/Skip".into(), "exclude".into());
    assert!(!state.get_folder_choices().row_data(0).unwrap().included);
    state.invoke_folder_tree("finish".into(), "".into(), "".into());
    assert!(!state.get_folder_dialog());
    match rx.try_recv().unwrap() {
        Request::AddFolder(root, rules) => {
            assert_eq!(root, "C:/Music");
            assert_eq!(rules, [("C:/Music/Skip".into(), "exclude".into())]);
        }
        _ => panic!("review must submit the selected rule with the root"),
    }
    state.set_folder_dialog(true);
    state.invoke_folder_tree("cancel".into(), "".into(), "".into());
    assert!(!state.get_folder_dialog());
    assert!(rx.try_recv().is_err());
    state.set_folder_adding(false);
    state.set_folder_rules(Rc::new(VecModel::default()).into());
}
