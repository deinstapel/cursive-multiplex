use std::process::Command;

#[test]
fn test_end2end() {
    assert!(
        Command::new("./node_modules/.bin/jest")
            .current_dir("./tests")
            .status()
            .expect("end-to-end tests failed")
            .success()
    );
}
