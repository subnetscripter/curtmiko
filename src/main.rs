#![allow(warnings)]

mod connections;
mod nodes;
mod net_io;

use nodes::base::{NodeInfo};
use nodes::fortigate::FortigateNode;
use nodes::juniper_srx::JuniperSrxNode;
use nodes::cisco_smb::CiscoSmbNode;
use std::error::Error;
use connections::ConnType;

use expectrl::{spawn, Regex, Eof};
    

fn main()  {

    let node_details = NodeInfo::new(
        "TEST-SMG",
        "10.0.0.66",
        22,
        "george",
        "george@123",
        ConnType::SSH
    );

    let mut node = CiscoSmbNode::new(node_details).unwrap();
    node.show_configs();

/*
        let mut cmd = std::process::Command::new("/usr/bin/ssh");
        cmd.args(["-o", "KexAlgorithms=+diffie-hellman-group1-sha1,diffie-hellman-group-exchange-sha1,diffie-hellman-group14-sha1",
             "-o", "HostKeyAlgorithms=+ssh-rsa,ssh-dss",
             "-o", "PubkeyAcceptedKeyTypes=+ssh-rsa,ssh-dss",
             "-p", "22", "10.0.0.66"]);
        eprintln!("{:?}", cmd);
        let mut conn = Session::spawn(cmd);

        match conn.expect(Eof) {
            Ok(found) => println!("ssh said: {}", String::from_utf8_lossy(found.as_bytes())),
            Err(e) => eprintln!("expect error: {e:?}"),
        }   


    let node_details = NodeInfo::new(
        "TEST-SMG",
        "10.0.0.66",
        22,
        "george",
        "george@123",
        ConnType::SSH
    );

    let mut smb_node = CiscoSmbNode::new(node_details)?;

    Ok(())
    let node_details = NodeInfo::new(
        "TEST-FORTIGATE",
        "10.0.0.1",
        22,
        "george",
        "george@123",
        ConnType::SSH
    );


    let mut fg_node = FortigateNode::new(node_details).unwrap();
    //fg_node.show_configs();
    fg_node.show_system_status();
    let srx_details = NodeInfo::new(
        "TEST-SRX",
        "192.168.1.3",
        22,
        "george",
        "george@123",
        ConnType::SSH
    );

    let mut srx_node = JuniperSrxNode::new(srx_details).unwrap();
    srx_node.show_system_status();
    //srx_node.show_configs();
*/

}


