
use std::process::{Command, Stdio};
use std::str;

fn main() {
    let interface = "Ethernet";
    let ethip = getip(interface);
    println!("{}",ethip);
    if ethip.trim().is_empty() {
       println!("no ip for interface: {}",interface);
      enablewifi(getwifidevice());
    } else {
        disablewifi(getwifidevice());
    }
}


fn getwifidevice() -> String{
    let mut wifiint = "".to_string();
    let listhard = Command::new("networksetup")
        .arg("-listallhardwareports")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = listhard.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    let wifiinttemp = result.split("\n");
    let mut nextline = 0;
    for line in wifiinttemp {
        if nextline == 1{
            wifiint = line.replace("Device: ","");
            return wifiint
        }
        if line.starts_with("Hardware Port: Wi-Fi"){
                nextline = 1;
        }
       
        
    }
    return wifiint

}
fn enablewifi(wifi_interface: String) {
    let checkwifi = Command::new("networksetup")
        .arg("-getairportpower")
        .arg(wifi_interface.clone())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = checkwifi.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    if result.contains("Off"){
    
        Command::new("networksetup")
        .arg("-setairportpower")
        .arg(wifi_interface.clone())
        .arg("on")
        .output()
        .expect("command failed to start");

        println!("wifi turned on");
    }else{
        println!("wifi already on");
    }
}

fn disablewifi(wifi_interface: String) {
    let checkwifi = Command::new("networksetup")
        .arg("-getairportpower")
        .arg(wifi_interface.clone())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = checkwifi.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    if result.contains("On"){
    
    Command::new("networksetup")
    .arg("-setairportpower")
    .arg(wifi_interface.clone())
    .arg("off")
    .output()
    .expect("command failed to start");

    println!("wifi turned off");
    }else{
        println!("wifi already off");
    }
}

fn getip(interface: &str) -> String{
    let mut ethip = "".to_string();
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
        if line.starts_with("IP address: "){
                ethip = line.replace("IP address: ","");
        }
    }
    return ethip
}
