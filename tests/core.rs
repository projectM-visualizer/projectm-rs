#[cfg(test)]
mod core {
    use projectm::core::*;
    use std::process::Command;
    use std::str;

    fn get_git_hash_by_command() -> Option<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .ok()?;

        if output.status.success() {
            let git_hash = str::from_utf8(&output.stdout).ok()?.trim().to_string();
            println!("git_hash: {}", git_hash);
            Some(git_hash)
        } else {
            None
        }
    }

    #[test]
    fn test_get_versions() {
        let version_tuple = ProjectM::get_version_components();
        assert_eq!(version_tuple, (4, 0, 0));

        let version_string = ProjectM::get_version_string();
        assert_eq!(version_string, "4.0.0");

        let vcs_version_string = ProjectM::get_vcs_version_string();
        assert_eq!(vcs_version_string, get_git_hash_by_command().unwrap());
    }

    // #[test]
    // fn test_create() {
    //     let projectm = ProjectM::create();
    // }
}
