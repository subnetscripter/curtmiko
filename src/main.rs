#![allow(warnings)]

mod connections;
mod nodes;
mod net_io;

use nodes::base::{NodeInfo};
use nodes::fortigate::FortigateNode;
use nodes::juniper_srx::JuniperSrxNode;
use std::error::Error;
use connections::ConnType;

    

fn main() {

/*
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
*/
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

}


