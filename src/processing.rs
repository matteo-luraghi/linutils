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
            handle,
            wheel: '|',
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
