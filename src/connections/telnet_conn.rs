use telnet::{Telnet, Event};
use regex::Regex;
use std::error::Error;
use std::time::Duration;




pub struct TelnetConn{
    pub conn: Telnet,
}

impl TelnetConn{
    pub fn build(ip: &str, port: u16) -> Result<TelnetConn, Box<dyn Error>> {

        //CONSIDER MAKING BUFFER SIZE A PASSABLE VALUE
        //INSTEAD OF HARD CODING 1024.
        let conn = Telnet::connect((ip, port), 1024)?;
        
        Ok(TelnetConn{conn})
    }



}

