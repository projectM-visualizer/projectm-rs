#[cfg(test)]
mod playlist {
    use projectm::core::ProjectM;
    use projectm::playlist::Playlist;

    #[test]
    fn playlist() {
        let projectm = ProjectM::create();
        let playlist = Playlist::create(&projectm);
        assert_eq!(playlist.is_empty(), true);

        // add ../presets to playlist
        // get absolute path to ../presets
        let path = std::env::current_dir().unwrap();
        let presets_dir = path.join("presets");
        playlist.add_path(presets_dir.to_str().unwrap(), true);
        assert_eq!(playlist.len(), 20);
    }
}
