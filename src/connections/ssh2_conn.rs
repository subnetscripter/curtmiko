use ssh2::{Session, Channel};
use std::error::Error;
use std::net::TcpStream;
use std::io::prelude::*;

pub struct SSHConn{
    pub sess: Session,
}



impl SSHConn {

    pub fn build(ip: &str, port: u16) ->  Result<SSHConn, Box<dyn Error>>{
    
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





}

