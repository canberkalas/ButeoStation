use std::process::Command;

pub fn check_ufw_status() {
    let output = Command::new("ufw")
        .arg("status")
        .output()
        .expect("Failed to execute UFW status check");

    let status = String::from_utf8_lossy(&output.stdout);
    if !status.contains("Status: active") {
        panic!("UFW is not active. Please enable UFW for security.");
    } else {
        println!("UFW is active.");
    }
}

pub fn check_open_ports() {
    let output = Command::new("lsof")
        .arg("-i")
        .output()
        .expect("Failed to check open ports");

    let ports = String::from_utf8_lossy(&output.stdout);
    println!("Open ports: \n{}", ports);
}
