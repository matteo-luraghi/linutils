use crate::tui::ProcessItem;
use std::{
    process::{Command, Stdio},
    thread,
};

/// Run a script from its path
fn exec_script(script_name: String) -> Result<String, String> {
    let mut script = Command::new("sh");
    let output = script
        .arg("-c")
        .arg(format!("sudo ./{}", script_name))
        // do not print the command's output on stdout
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect(&format!("Error executing script {}", script_name));

    if script.status().expect("Failed to execute script").success() {
        return Ok(format!("Script {} executed correctly", script_name));
    } else {
        return Err(format!(
            "Error executing script {}\n{:?}",
            script_name, output
        ));
    }
}

/// Run all scripts in a Vector each on a separate thread
pub fn run_all(packages: Vec<String>) -> Vec<ProcessItem> {
    // save each thread's handle in a vector
    let mut process_items = vec![];

    for package in packages {
        let package_thread = package.clone();
        let handle = thread::spawn(move || {
            let result = exec_script(package_thread + ".sh");
            return result;
        });

        let process_item = ProcessItem {
            name: package,
            handle: Some(handle),
            wheel: '|',
            is_finished: false,
            error_message: "".to_string(),
        };

        process_items.push(process_item);
    }

    process_items

    /*
        // save each thread's result in a vector
        let mut results: Vec<Result<String, String>> = vec![];
        for handle in process_items {
            let result = handle.join();
            results.push(result.unwrap());
        }

        results
    */
}

#[test]
fn test_exec_script() {
    let scripts = ["src/commands/test.sh", "src/commands/bad-test.sh"];
    let expected_outputs = [
        Ok("Script src/commands/test.sh executed correctly".to_string()), 
        Err("Error executing script src/commands/bad-test.sh\nOutput { status: ExitStatus(unix_wait_status(256)), stdout: \"\", stderr: \"test crushed\\n\" }".to_string())];
    let mut outputs: Vec<Result<String, String>> = vec![];

    for script in scripts {
        let output = exec_script(script.to_string());
        outputs.push(output);
    }

    let mut i = 0;
    for output in outputs {
        assert_eq!(output, expected_outputs[i]);
        i += 1;
    }
}
