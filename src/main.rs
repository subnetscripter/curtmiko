#![allow(warnings)]

mod connections;
mod nodes;
mod net_io;

use regex::Regex;
use nodes::base::{TelnetNode, SSHNode};
use std::error::Error;


fn main() {
    
    //SSH testing
    let mut node1 = SSHNode::build("10.0.0.28", 22).unwrap();
    node1.authenticate("george", "george@123").unwrap();
    node1.send_command("ls -lh /").unwrap();

    //Telnet testing
    let mut node = TelnetNode::build("10.0.0.66", 23).unwrap();
    println!("Telnet connection to node succesful!");
    let mut reg = Regex::new(r":$").unwrap();
    let _ = node.authenticate("george", "george@123", &reg);
    println!("Authenticaton to node successful");
    let mut reg = Regex::new(r"#").unwrap();
    node.send_command("sh run", &reg);
}


