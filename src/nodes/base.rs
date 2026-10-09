use std::error::Error;
use regex::Regex;

use crate::connections::telnet_conn::TelnetConn;
use crate::connections::ssh2_conn::SSHConn;
use crate::net_io::reader::Reader;
use crate::net_io::writer::Writer;


/*

This crate is the abstration layer into the connectivity functionality
of the library. The idea is to only expose users to this crate, so we can
manipulate the backend without causing breaking changes.

*/


//Node will be a generic struct type that connects to any device
//Right now it only connects via telnet.
//Need to expand to ssh for secure connections in the future.
pub struct TelnetNode{
    conn: TelnetConn,
    reader: Reader,
    writer: Writer
}

impl TelnetNode{
    pub fn build(ip: &str, port: u16) -> Result<TelnetNode, Box<dyn Error>> {
        
        let telnet_conn = TelnetConn::build(ip, port)?;
        let reader = Reader::new();
        let writer = Writer::new();


        Ok(TelnetNode{conn: telnet_conn, reader, writer})
    }



    // #BUG DETECTED#
    //If the user credentials do not exist on the remote node
    //The terminal hangs. Need to create error handling for this situation.
    pub fn authenticate(&mut self, username: &str, password: &str, reg: &Regex) -> Result<(), Box<dyn Error>> {
        self.reader.read_from_telnet(&mut self.conn, &reg);
        let _ = self.writer.write_to_telnet(&mut self.conn, username)?;

        self.reader.read_from_telnet(&mut self.conn, &reg);
        let _ = self.writer.write_to_telnet(&mut self.conn, password)?;

        Ok(())

    }

    pub fn send_command(&mut self, command: &str, reg: &Regex) -> Result<(), Box<dyn Error>> {
        //Looks for the correct ending prompt to ensure that you can send the required command.
        self.reader.read_from_telnet(&mut self.conn, reg);
        match self.writer.write_to_telnet(&mut self.conn, command) {
            Ok(t) => println!("COMMAND: {command}"),
            Err(e) => return Err(e)
        }
        self.reader.read_from_telnet(&mut self.conn, reg);
        Ok(())
    }

}






pub struct SSHNode{
    conn: SSHConn,
    reader: Reader,
    writer: Writer
}

impl SSHNode{
    pub fn build(ip: &str, port: u16) -> Result<SSHNode, Box<dyn Error>>{
        let conn = SSHConn::build(ip, port)?;
        let reader = Reader::new();
        let writer = Writer::new();
        Ok(SSHNode{conn, reader, writer}) 
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>>{
        let _ = self.conn.sess.userauth_password(username, password)?;
        Ok(())
    }

    pub fn send_command(&mut self, string_data: &str) -> Result<(), Box<dyn Error>>{
        let _ = self.writer.write_to_ssh(&mut self.conn, string_data, &self.reader)?;
        Ok(())
        
    }
}
