use crate::connections::telnet_conn::TelnetConn;
use crate::connections::ssh2_conn::SSHConn;
use crate::net_io::reader::Reader;
use std::error::Error;
use regex::Regex;
use std::time::Duration;
use telnet::Event;



//Unit struct used to implement functions to write data for various conn types.
pub struct Writer;


impl Writer{
    
    pub fn new() -> Writer{
        Writer
    }

    pub fn write_to_telnet(&self, telnet_conn: &mut TelnetConn, string_data: &str) -> Result<(), Box<dyn Error>>{
        
        let data = string_data.as_bytes();
        let mut result = telnet_conn.conn.write(data)?;
        result = telnet_conn.conn.write(b"\n")?;
        Ok(())

    }

    pub fn write_to_ssh(&self, ssh_conn: &mut SSHConn, string_data: &str, reader: &Reader) -> Result<(), Box<dyn Error>> {

        let mut chan = ssh_conn.sess.channel_session()?;
        let _ = chan.exec(string_data)?;
        let _ = reader.read_from_ssh(&mut chan)?;

        Ok(())

    }


}
