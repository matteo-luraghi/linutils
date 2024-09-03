use crate::{
    config::{load_config, Distro},
    tui::ProcessItem,
};
use std::{
    fs, io,
    process::{Command, Stdio},
    sync::Arc,
    thread,
};

/// Try to install the package using the package manager of the selected distro
fn default_installation(
    distro: String,
    package: String,
    config_distros: Vec<Distro>,
) -> Result<String, String> {
    // find the correct package manager for the selected distro
    let package_manager = match config_distros
        .iter()
        .find(|d| d.name == distro)
        .map(|d| d.package_manager.clone())
    {
        Some(p) => p,
        None => return Err("Distro not supported".to_string()),
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

/// Run a script in the commands directory under its distro directory
fn exec_script(distro: String, script_name: String) -> Result<String, String> {
    let mut script = Command::new("sh");
    let output = script
        .arg("-c")
        .arg(format!("sudo ./src/commands/{}/{}", distro, script_name))
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
    } else {
        return Err(format!(
            "Error executing script {} {:?}",
            script_name, output
        ));
    }
}

/// Get all the files in a directory
fn get_files_in_directory(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;

    let file_names: Vec<String> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(file_names)
}

/// Check if a specific script is present
fn is_script_present(scripts: &io::Result<Vec<String>>, script: String) -> bool {
    let target = format!("{}.sh", script);
    match scripts {
        Ok(vec) => vec.iter().any(|file| file == &target),
        Err(_) => false,
    }
}

/// Run all scripts in a Vector each on a separate thread
pub fn run_all(packages: Vec<String>, distros: Vec<String>) -> Vec<ProcessItem> {
    let config = load_config("./src/config.toml");

    // get the only distro selected
    let distro = distros.get(0).unwrap();

    // get all the available scripts
    let scripts = get_files_in_directory(format!("src/commands/{}/", distro).as_str());

    // save each thread's handle in a vector
    let mut process_items = vec![];

    // share the distros in multiple threads
    let distros_arc = Arc::new(config.distros);

    for package in packages {
        let package_thread = package.clone();
        let distro_thread = distro.clone();
        let handle;
        // check if installation script is present
        if is_script_present(&scripts, package.clone()) {
            // use the specific script
            handle = thread::spawn(move || {
                let result = exec_script(distro_thread, package_thread + ".sh");
                return result;
            });
        } else {
            // safely clone the distros vector
            let distros_arc_clone = Arc::clone(&distros_arc);
            handle = thread::spawn(move || {
                // use the default installation via package manager
                let result =
                    default_installation(distro_thread, package_thread, distros_arc_clone.to_vec());
                return result;
            });
        }

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
fn test_run_all() {
    let packages = vec!["test".to_string(), "bad-test".to_string()];
    let distros = vec!["test".to_string()];

    let result = run_all(packages.clone(), distros);

    let mut i = 0;
    for item in result {
        assert_eq!(item.name, packages.get(i).unwrap().clone());
        assert_eq!(item.wheel, '|');
        assert_eq!(item.is_finished, false);
        assert_eq!(item.error_message, "".to_string());
        i += 1;
    }
}

#[test]
fn test_exec_script() {
    let scripts = ["test.sh", "bad-test.sh"];
    let expected_outputs = [
        Ok("Script test.sh executed correctly".to_string()), 
        Err("Error executing script bad-test.sh Output { status: ExitStatus(unix_wait_status(256)), stdout: \"\", stderr: \"test crushed\\n\" }".to_string())];
    let mut outputs: Vec<Result<String, String>> = vec![];

    for script in scripts {
        let output = exec_script("_tests".to_string(), script.to_string());
        outputs.push(output);
    }

    let mut i = 0;
    for output in outputs {
        assert_eq!(output, expected_outputs[i]);
        i += 1;
    }
}
