use crate::connections::ConnType;
use ssh2::MethodType;
use crate::nodes::base::{ExpectNode, NodeInfo};
use std::error::Error;
use regex::Regex;
use std::process::Command;

const EXPECT_SCRIPT: &str = "/home/curtley/RUST/curtmiko/src/nodes/cisco_smb/expect_ssh";
const SSH_SCRIPT: &str = "/home/curtley/RUST/curtmiko/src/nodes/cisco_smb/ssh_conn.sh";

const SmbLoginPromptReg: &str = ":$";

pub struct CiscoSmbNode{
    node: NodeInfo
}

impl CiscoSmbNode{
    
    pub fn new(node_info: NodeInfo) -> Result<CiscoSmbNode, Box<dyn Error>>{
        Ok(CiscoSmbNode{node: node_info})

    }




    pub fn show_configs(&mut self){
        let output = Command::new(EXPECT_SCRIPT)
            .arg(self.node.ip())
            .arg(format!("{}",self.node.port()))
            .arg(self.node.username())
            .arg(self.node.password())
            .arg("show run")
            .arg(SSH_SCRIPT)
            .output()
            .expect("Failed to run script");

        let status = output.status;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{stdout}");
    }

    pub fn show_system_status(&mut self){
        ////
    }

}
