pub mod telnet_conn;
pub mod ssh2_conn;

use telnet_conn::TelnetConn;
use std::error::Error;
use regex::Regex;
use std::time::Duration;
use telnet::Event;


trait Conn{

    fn write_data(&mut self) -> Result<(), Box<dyn Error>>;


}

