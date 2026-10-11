use crate::connections::ConnType;
use crate::nodes::base::{NodeType, SSHNode, TelnetNode, NodeInfo};
use std::error::Error;
use regex::Regex;


const REMOVE_PAGINATION: &str = "conf system console\nset output standard\nend\n";

pub struct FortigateNode{
    node: NodeType,
}

impl FortigateNode{
    pub fn new(node_info: NodeInfo ) -> Result<FortigateNode, Box<dyn Error>>{
        match node_info.conn_type(){
            ConnType::SSH =>{
                let mut node = SSHNode::build(node_info.ip(), node_info.port())?;
                let _ = node.authenticate(node_info.username(), node_info.password())?;
                println!("Authentication to fortigate node successful!");
                Ok(FortigateNode {node: NodeType::SSH(node)})
            },
            ConnType::Telnet =>{
                let mut node = TelnetNode::build(node_info.ip(), node_info.port())?;
                let reg = Regex::new(r":$").unwrap();
                let _ = node.authenticate(node_info.username(), node_info.password(), &reg)?;
                Ok(FortigateNode{node: NodeType::Telnet(node)})
            }

        }

    }

    pub fn show_configs(&mut self){
        transport_send("show full-configuration", self)
    }

    pub fn show_system_status(&mut self){
        transport_send("get system status", self);
    }
}

//Sends commands based on the transport medium.
//ie. SSH or Telnet
fn transport_send(command: &str, fg_node: &mut FortigateNode){
    
    match fg_node.node {
        NodeType::SSH(ref mut sn) => {
            let _ = sn.send_command(REMOVE_PAGINATION).unwrap();
            let _ = sn.send_command(command).unwrap();
        },
        NodeType::Telnet(ref mut tn) => {
            let reg = Regex::new(r"#$").unwrap();
            let _ = tn.send_command(REMOVE_PAGINATION, &reg).unwrap();
            let _ = tn.send_command("show full-configuration", &reg).unwrap();
        }
    }
}
