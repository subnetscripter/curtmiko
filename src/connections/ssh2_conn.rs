use ssh2::{Session, Channel};
use regex::Regex;
use std::error::Error;
use std::time::Duration;
use std::net::TcpStream;
use std::io::prelude::*;

pub struct SSHConn{
    sess: Session,
}



impl SSHConn {

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {

        let _ = self.sess.userauth_password(username, password)?;
        
        Ok(())

    }


    pub fn build(ip: &str, port: u16) -> Result<SSHConn, Box<dyn Error>>{
    
        //Create SSH session
        let mut sess = Session::new()?;


        //NEED TO ADD VALIDATION SO THAT 0 < port < 65535
        let port_as_string = port.to_string();
        let socket = format!("{ip}:{port_as_string}");

        //Create TCP stream to destination and wraps it in the session.
        let tcp_stream = TcpStream::connect(socket)?;
        sess.set_tcp_stream(tcp_stream);
        sess.handshake().unwrap();

        Ok(SSHConn{sess})
    }


    //ADD MISSING REGEX PARAM LATER
    pub fn read_data(&mut self, chan: &mut Channel) -> Result<(), Box<dyn Error>> {
        
        //Used to store output returned from the channel.
        let mut s = String::new();
        
        let _ = chan.read_to_string(&mut s)?;

        println!("{}", s);

        let _ = chan.wait_close()?;

        println!("{}", chan.exit_status().unwrap());

        Ok(())

    }

    pub fn write_data(&mut self, data: &str) -> Result<(), Box<dyn Error>>{
        
        let mut chan = self.sess.channel_session()?;
        let _ = chan.exec(data)?;
        let _ = self.read_data(&mut chan)?;

        Ok(())

    }
}

