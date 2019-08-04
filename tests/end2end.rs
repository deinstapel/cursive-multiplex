use serde_json::{json, Value};
use std::fs;
use std::process::{Command, Stdio};

#[test]
fn test_end2end() {
    let jest = Command::new("./node_modules/.bin/jest")
        .arg("--color")
        .arg("--json")
        .current_dir("./tests")
        .stderr(Stdio::inherit())
        .output()
        .expect("end2end tests failed");

    assert!(jest.status.success());

    let result: Value =
        serde_json::from_slice(&jest.stdout[..]).expect("failed to read end2end tests result");
    let passed = result["numPassedTests"].as_i64().unwrap();
    let total = result["numTotalTests"].as_i64().unwrap();
    let color = match passed as f32 / total as f32 {
        r if r < 0.33 => "red",
        r if r < 0.66 => "orange",
        r if r < 1.0 => "yellow",
        _ => "brightgreen",
    };

    let shield = json!({
        "schemaVersion": 1,
        "label": "shellshot",
        "message": format!("{} / {} tests", passed, total),
        "color": color,
        "isError": true
    });
    fs::create_dir_all("./target/shields").expect("failed to create ./target/shields folders");
    fs::write(
        "./target/shields/shellshot.json",
        serde_json::to_string_pretty(&shield).expect("failed to generate shields endpoint"),
    )
    .expect("failed to save shields endpoint");
}
