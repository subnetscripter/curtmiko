#![allow(warnings)]

mod connections;
mod nodes;
mod net_io;

use regex::Regex;
use nodes::base::Node;
use connections::ssh2_conn::SSHConn;
use std::error::Error;


fn main() {
    
    let mut ssh_conn = SSHConn::build("10.0.0.28", 22).unwrap();
    let  _ = ssh_conn.authenticate("george","george@123");
    let  _ = ssh_conn.write_data("ls -lh /");

/*
    let mut node = Node::build("10.0.0.66", 23).unwrap();
    println!("Telnet connection to node succesful!");

    let mut reg = Regex::new(r":$").unwrap();
    let _ = node.authenticate("george", "george@123", &reg);
    println!("Authenticaton to node successful");
    let mut reg = Regex::new(r"#").unwrap();
    node.send_command("sh run", &reg);
*/
}


