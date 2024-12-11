
use std::process::{Command, Stdio};
use std::str;

fn main() {
    let interface = "Ethernet";
    let ethip = getip(interface);
    println!("{}",ethip);
if ethip.trim().is_empty() {
    println!("no ip for interface: {}",interface);
    enablewifi()
} else {
    disablewifi()
}

}
fn enablewifi() {
    Command::new("networksetup")
    .arg("-setairportpower")
    .arg("wifi")
    .arg("on")
    .output()
    .expect("command failed to start");

    println!("wifi turned on")
}

fn disablewifi() {
    Command::new("networksetup")
    .arg("-setairportpower")
    .arg("wifi")
    .arg("off")
    .output()
    .expect("command failed to start");

    println!("wifi turned off")
}

fn getip(interface: &str) -> String{
    let ps_child = Command::new("networksetup") // `ps` command...
        .arg("-getinfo")                  // with argument `axww`...
        .arg(interface)                  // with argument `axww`...
        .stdout(Stdio::piped())       // of which we will pipe the output.
        .spawn()                      // Once configured, we actually
                                      // spawn the command...
        .unwrap();                    // and assert everything went right.
    let grep_child_one = Command::new("grep")
        .arg(r#"^IP address"#)
        .stdin(Stdio::from(ps_child.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_two = Command::new("awk")
        .arg(r"-F:")
        .arg(r#"{print $2}"#)
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_two.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    let ethip = result.replace(" ", "");
    return ethip
}