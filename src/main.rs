
use std::process::{Command, Stdio};
use std::str;

fn main() {
    let interface = "Ethernet";
    let ethip = getip(interface);
    println!("{}",ethip);
if ethip.trim().is_empty() {
    println!("no ip for interface: {}",interface);
    enablewifi();
} else {
    disablewifi();
}

}
fn enablewifi() {
    Command::new("networksetup")
    .arg("-setairportpower")
    .arg("wifi")
    .arg("on")
    .output()
    .expect("command failed to start");

    println!("wifi turned on");
}

fn disablewifi() {
    Command::new("networksetup")
    .arg("-setairportpower")
    .arg("wifi")
    .arg("off")
    .output()
    .expect("command failed to start");

    println!("wifi turned off");
}

fn getip(interface: &str) -> String{
    let grep_child_two = Command::new("networksetup")
        .arg("-getinfo")
        .arg(interface)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_two.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    let ethiptemp = result.split("\n");
    for line in ethiptemp {
        if line.contains("IP address: "){
                let ethip = line.replace("IP address: ","");
                //println!("{}", ethip)
        }
    }
    return ethip //to_string()
}
