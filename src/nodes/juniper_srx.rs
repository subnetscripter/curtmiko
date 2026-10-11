use crate::connections::ConnType;
use crate::nodes::base::{NodeType, SSHNode, TelnetNode, NodeInfo};
use std::error::Error;
use regex::Regex;



pub struct JuniperSrxNode{
    node: NodeType,
}

impl JuniperSrxNode{
    pub fn new(node_info: NodeInfo ) -> Result<JuniperSrxNode, Box<dyn Error>>{
        match node_info.conn_type(){
            ConnType::SSH =>{
                let mut node = SSHNode::build(node_info.ip(), node_info.port())?;
                let _ = node.authenticate(node_info.username(), node_info.password())?;
                println!("Authentication to Juniper node successful!");
                Ok(JuniperSrxNode {node: NodeType::SSH(node)})
            },
            ConnType::Telnet =>{
                let mut node = TelnetNode::build(node_info.ip(), node_info.port())?;
                let reg = Regex::new(r":$").unwrap();
                let _ = node.authenticate(node_info.username(), node_info.password(), &reg)?;
                Ok(JuniperSrxNode{node: NodeType::Telnet(node)})
            }

        }

    }

    pub fn show_configs(&mut self){
        transport_send("show configuration | display set | no-more", self)
    }

    pub fn show_system_status(&mut self){
        transport_send("show chassis hardware | no-more", self);
        transport_send("show system uptime | no-more", self);
    }
}

//Sends commands based on the transport medium.
//ie. SSH or Telnet
/*
CODE IN TELNET IS JUST A PLACEHODER.
IT DEFINITELY NEEDS TO BE REFACTORED FOR SPECIFIC DEVICE.
WAS FEELING TO LAZY TO DO IT RIGHT NOW.
*/
fn transport_send(command: &str, fg_node: &mut JuniperSrxNode){
    
    match fg_node.node {
        NodeType::SSH(ref mut sn) => {
            let _ = sn.send_command(command).unwrap();
        },
        NodeType::Telnet(ref mut tn) => {
            let reg = Regex::new(r"#$").unwrap();
            let _ = tn.send_command(command, &reg).unwrap();
        }
    }
}
