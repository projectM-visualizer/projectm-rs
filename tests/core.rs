#[cfg(test)]
mod core {
    use projectm_rs::core::projectm;

    #[test]
    fn test_get_versions() {
        let version_tuple = projectm::get_version_components();
        assert_eq!(version_tuple, (4, 0, 0));

        let version_string = projectm::get_version_string();
        assert_eq!(version_string, "4.0.0");

        let vcs_version_string = projectm::get_vcs_version_string();
        // assert_eq!(vcs_version_string, "$COMMITHASH$");
    }

    // #[test]
    // fn test_sample() {
    //     let projectm = projectm::create();

    //     projectm::sample(projectm, 60);
    //     assert_eq!(projectm, sample);
    // }
}
