use std::process::{Command, Stdio};

pub fn exec_script(path: &str) {
    let script_name_optional = path.split("/").last();

    let script_name = match script_name_optional {
        Some(name) => name,
        None => {
            println!("Failed to extract script name");
            return;
        }
    };

    let mut script = Command::new("sh");
    let output = script
        .arg("-c")
        .arg(format!("./{}", path))
        .stdout(Stdio::piped())
        .output()
        .expect(&format!("Error executing script {}", script_name));

    if script.status().expect("Failed to execute script").success() {
        println!("Command executed correctly");
    } else {
        println!("Error executing script {}", script_name);
        println!("{:?}", output);
    }
}
