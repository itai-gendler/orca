use super::*;

fn wave(path: &Path) {
    let samples = [0u8; 1600];
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + samples.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&8000u32.to_le_bytes());
    bytes.extend_from_slice(&16000u32.to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&(samples.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&samples);
    fs::write(path, bytes).unwrap();
}

fn scan(b: &mut Backend, root: &Path) {
    b.start_scan(root.to_str().unwrap()).unwrap();
    while b.snapshot().unwrap().scanning {
        thread::sleep(Duration::from_millis(5));
    }
    b.cancel_and_join_scan();
    assert!(b.snapshot().unwrap().error.is_empty());
}

#[test]
fn nested_rules_preserve_likes_playlist_membership_and_song_identity() {
    let f = Fixture::new();
    let root = f.0.join("music");
    let excluded = root.join("skip");
    let included_child = excluded.join("keep");
    let adjacent = root.join("skip-extra");
    for dir in [&included_child, &adjacent] {
        fs::create_dir_all(dir).unwrap();
    }
    let skip_song = excluded.join("old.wav");
    let keep_song = included_child.join("keep.wav");
    let adjacent_song = adjacent.join("adjacent.wav");
    for path in [&skip_song, &keep_song, &adjacent_song] {
        wave(path);
    }
    let root = root.canonicalize().unwrap();
    let excluded = excluded.canonicalize().unwrap();
    let included_child = included_child.canonicalize().unwrap();
    let skip_song = skip_song.canonicalize().unwrap();
    let keep_song = keep_song.canonicalize().unwrap();
    let adjacent_song = adjacent_song.canonicalize().unwrap();
    let mut b = f.backend();
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 3);
    let id: i64 = b
        .conn
        .query_row(
            "SELECT id FROM songs WHERE path=?1",
            [skip_song.to_str().unwrap()],
            |r| r.get(0),
        )
        .unwrap();
    let playlist = db::create_playlist(&b.conn, "Keep membership", None).unwrap();
    db::add_to_playlist(&b.conn, playlist, id).unwrap();
    db::set_song_liked(&b.conn, id, true).unwrap();

    b.set_folder_rule(excluded.to_str().unwrap(), "exclude")
        .unwrap();
    assert_eq!(b.statistics().unwrap().songs, 1);
    assert_eq!(db::liked_songs_summary(&b.conn).unwrap().count, 0);
    assert_eq!(db::get_playlists(&b.conn).unwrap()[0].song_count, 0);
    assert!(b.track(skip_song.to_str().unwrap()).is_err());
    assert!(db::is_song_liked(&b.conn, id).unwrap());
    assert_eq!(db::get_song_playlist_ids(&b.conn, id).unwrap(), [playlist]);
    fs::remove_file(&skip_song).unwrap();
    scan(&mut b, &root);
    assert_eq!(
        b.conn
            .query_row(
                "SELECT id FROM songs WHERE path=?1",
                [skip_song.to_str().unwrap()],
                |r| r.get::<_, i64>(0)
            )
            .unwrap(),
        id
    );
    wave(&skip_song);
    b.set_folder_rule(included_child.to_str().unwrap(), "include")
        .unwrap();
    assert_eq!(b.statistics().unwrap().songs, 2);
    assert!(b.track(keep_song.to_str().unwrap()).is_ok());
    assert!(b.track(adjacent_song.to_str().unwrap()).is_ok());
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 2);

    let new_skip = excluded.join("new");
    let new_keep = included_child.join("new");
    fs::create_dir_all(&new_skip).unwrap();
    fs::create_dir_all(&new_keep).unwrap();
    wave(&new_skip.join("hidden.wav"));
    wave(&new_keep.join("visible.wav"));
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 3);
    assert!(b
        .conn
        .query_row(
            "SELECT id FROM songs WHERE path=?1",
            [new_skip.join("hidden.wav").to_str().unwrap()],
            |r| r.get::<_, i64>(0)
        )
        .is_err());
    assert!(b
        .track(new_keep.join("visible.wav").to_str().unwrap())
        .is_ok());

    b.set_folder_rule(excluded.to_str().unwrap(), "inherit")
        .unwrap();
    assert_eq!(b.statistics().unwrap().songs, 4);
    assert_eq!(b.track(skip_song.to_str().unwrap()).unwrap().id, id);
    assert_eq!(db::liked_songs_summary(&b.conn).unwrap().count, 1);
    assert_eq!(db::get_playlists(&b.conn).unwrap()[0].song_count, 1);
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 5);
}

#[test]
fn folder_preview_and_initial_rules_are_validated_before_registration() {
    let f = Fixture::new();
    let root = f.0.join("music");
    let nested = root.join("nested");
    let outside = f.0.join("music-other");
    fs::create_dir_all(&nested).unwrap();
    fs::create_dir_all(&outside).unwrap();
    wave(&nested.join("excluded.wav"));
    wave(&root.join("direct.wav"));
    let mut b = f.backend();
    let preview = b
        .folder_children(root.to_str().unwrap(), Some(root.to_str().unwrap()))
        .unwrap();
    assert_eq!(preview.len(), 1);
    assert_eq!(preview[0].explicit, "inherit");
    assert!(preview[0].included);
    assert!(b
        .folder_children(outside.to_str().unwrap(), Some(root.to_str().unwrap()))
        .is_err());
    assert!(b
        .add_folder_with_rules(
            root.to_str().unwrap(),
            &[(outside.to_string_lossy().into_owned(), "exclude".into())]
        )
        .is_err());
    assert!(b.statistics().unwrap().roots.is_empty());
    b.add_folder_with_rules(
        root.to_str().unwrap(),
        &[(nested.to_string_lossy().into_owned(), "exclude".into())],
    )
    .unwrap();
    assert!(!b.folder_children(root.to_str().unwrap(), None).unwrap()[0].included);
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 1);
    assert_eq!(
        b.conn
            .query_row("SELECT COUNT(*) FROM songs", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        1
    );
    assert!(b
        .set_folder_rule(outside.to_str().unwrap(), "include")
        .is_err());
    assert!(b
        .set_folder_rule(root.to_str().unwrap(), "exclude")
        .is_err());
    assert!(b
        .set_folder_rule(nested.to_str().unwrap(), "invalid")
        .is_err());
    b.operation(&serde_json::json!({"action":"remove-source","key":root}).to_string())
        .unwrap();
    assert_eq!(
        b.conn
            .query_row("SELECT COUNT(*) FROM folder_rules", [], |r| r
                .get::<_, i64>(0))
            .unwrap(),
        0
    );
}

#[test]
fn initially_excluded_folder_is_scanned_after_it_is_included() {
    let f = Fixture::new();
    let root = f.0.join("music");
    let genres = root.join("Genres");
    fs::create_dir_all(&genres).unwrap();
    wave(&root.join("visible.wav"));
    wave(&genres.join("new.wav"));
    let root = root.canonicalize().unwrap();
    let genres = genres.canonicalize().unwrap();
    let mut b = f.backend();
    b.add_folder_with_rules(
        root.to_str().unwrap(),
        &[(genres.to_string_lossy().into_owned(), "exclude".into())],
    )
    .unwrap();
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 1);
    b.set_folder_rule(genres.to_str().unwrap(), "include")
        .unwrap();
    assert_eq!(b.statistics().unwrap().songs, 1);
    scan(&mut b, &root);
    assert_eq!(b.statistics().unwrap().songs, 2);
    assert!(b.track(genres.join("new.wav").to_str().unwrap()).is_ok());
}
