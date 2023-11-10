use std::process::Command;

fn main() {
    let npm_install_status = Command::new("node")
        .arg("-e")
        .arg("require('child_process').execSync('npm install @oogaboogagames/game-core@latest')")
        .status()
        .expect("Failed to run npm install");

    if !npm_install_status.success() {
        panic!("NPM installation failed");
    }
}
