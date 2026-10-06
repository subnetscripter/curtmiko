#![allow(warnings)]

mod connections;
mod nodes;

use regex::Regex;
use nodes::base::Node;

fn main() {
    
    let mut node = Node::build("10.0.0.66", 23).unwrap();
    println!("Telnet connection to node succesful!");

    let mut reg = Regex::new(r":$").unwrap();
    let _ = node.authenticate("george", "george@123", &reg);
    println!("Authenticaton to node successful");

    let mut reg = Regex::new(r"#$").unwrap();
    node.send_command("conf t", &reg);
    node.send_command("username timothy password Grimmjow@1234", &reg);
    node.send_command("username Aimothy password Grimmjow@1234", &reg);
    node.send_command("username Cimothy password Grimmjow@1234", &reg);
    
}


