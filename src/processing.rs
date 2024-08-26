use std::{
    process::{Command, Stdio},
    thread,
};

fn exec_script(path: String) -> Result<String, String> {
    let script_name_optional = path.split("/").last();

    let script_name = match script_name_optional {
        Some(name) => name,
        None => {
            return Err("Failed to extract script name".to_string());
        }
    };

    let mut script = Command::new("sh");
    let output = script
        .arg("-c")
        .arg(format!("./{}", path))
        // do not print the command's output on stdout
        .stdout(Stdio::piped())
        .output()
        .expect_err(&format!("Error executing script {}", script_name));

    if script.status().expect("Failed to execute script").success() {
        return Ok(format!("Script {} executed correctly", script_name));
    } else {
        return Err(format!(
            "Error executing script {}\n{:?}",
            script_name, output
        ));
    }
}

pub fn run_all(paths: Vec<String>) -> Vec<Result<String, String>> {
    // save each thread's handle in a vector
    let mut handles = vec![];

    for path in paths {
        let handle = thread::spawn(|| {
            let result = exec_script(path);
            return result;
        });

        handles.push(handle);
    }

    // save each thread's result in a vector
    let mut results: Vec<Result<String, String>> = vec![];
    for handle in handles {
        let result = handle.join();
        results.push(result.unwrap());
    }

    results
}

#[test]
fn test_run_all() {
    let packages = vec!["alacritty".to_string(), "src/commands/test.sh".to_string()];
    let expected_output = vec![
    Err("Error executing script alacritty\nOutput { status: ExitStatus(unix_wait_status(32512)), stdout: \"\", stderr: \"sh: 1: ./alacritty: not found\\n\" }".to_string()),
    Ok("Script test.sh executed correctly".to_string())];

    let results = run_all(packages);
    assert_eq!(results, expected_output);
}
