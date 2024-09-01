use crate::tui::ProcessItem;
use std::{
    process::{Command, Stdio},
    thread,
};

fn default_installation(distro: String, package: String) -> Result<String, String> {
    let package_manager = match distro.as_str() {
        "fedora" => "dnf".to_string(),
        "ubuntu" => "nala".to_string(),
        _ => return Err("Distro not supported".to_string()),
    };

    let mut command = Command::new("sh");
    let output = command
        .arg("-c")
        .arg(format!("sudo {} install {} -y", package_manager, package))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect(&format!("Failed to install package {}", package));

    if command
        .status()
        .expect(&format!("Failed to install package {}", package))
        .success()
    {
        return Ok(format!("Package {} installed successfully", package));
    } else {
        return Err(format!("Error installing package {} {:?}", package, output));
    }
}

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
        .expect(&format!("Failed to execute script: {}", script_name));

    if script
        .status()
        .expect(&format!("Failed to execute script: {}", script_name))
        .success()
    {
        return Ok(format!("Script {} executed correctly", script_name));
    } else if String::from_utf8_lossy(&output.stderr).contains("not found") {
        // try to install the package via the package manager
        return default_installation("ubuntu".to_string(), script_name.clone());
    } else {
        return Err(format!(
            "Error executing script {} {:?}",
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
