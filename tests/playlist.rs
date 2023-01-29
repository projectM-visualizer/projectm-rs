// use projectm_rs::core::projectm;
// use projectm_rs::playlist::Playlist;

#[cfg(test)]
mod tests {
    #[test]
    fn playlist() {
        let projectm = projectm_rs::core::projectm::create();
        let playlist = projectm_rs::playlist::Playlist::create(projectm);
        assert_eq!(playlist.len(), 0);

        // add ../presets to playlist
        // get absolute path to ../presets
        let path = std::env::current_dir().unwrap();
        let presets_dir = path.join("presets");
        playlist.add_path(presets_dir.to_str().unwrap(), true);
        assert_eq!(playlist.len(), 20);
    }
}
