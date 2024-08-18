use std::env::args;
use std::process::Command;

fn main() {
    let distro = args().nth(1).expect("Error: no distro selected");
    println!("{}", distro);

    let script_path = format!("./{}/apps/discord.sh", distro);

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("yes | sudo {}", script_path)) // repeatedly says yes to every prompt
        .output()
        .expect("failed to execute process");
    println!("{:?}", output)
}
