use std::env::args;
use std::process::Command;

fn main() {
    let distro = args().nth(1).expect("Error: no distro selected");
    println!("{}", distro);

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");
    println!("{:?}", output)
}
