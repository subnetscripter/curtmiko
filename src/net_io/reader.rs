use crate::connections::telnet_conn::TelnetConn;
use crate::connections::ssh2_conn::SSHConn;
use std::error::Error;
use regex::Regex;
use std::time::Duration;
use telnet::Event;
use ssh2::Channel;
use std::io::Read;


pub struct Reader;

impl Reader{

    pub fn new() -> Reader {
        return Reader
    }

    pub fn read_from_telnet(&self, telnet_conn: &mut TelnetConn, reg: &Regex) -> Result<(), Box<dyn Error>> {

        //Used to buffer the output from reading the telnet 
        let mut hay_buffer = String::new();

        //Loops over the telnet Even stream until the even variant
        //we want (Event::Data) is found
        loop{
            match telnet_conn.conn.read_timeout(Duration::from_secs(2)){
                Ok(Event::Data(data)) => {
                    let data_str = String::from_utf8_lossy(&data);
                    hay_buffer.push_str(data_str[..].trim());
                    match reg.find(&hay_buffer[..]){
                        Some(t) => {
                            println!("{hay_buffer\n}"); //THIS LINE TO BE REMOVED
                            break
                        },
                        None => continue
                    }
                },
                _ => continue
            };
        };
        Ok(())
    }




    pub fn read_from_ssh(&self, chan: &mut Channel) -> Result<(), Box<dyn Error>>{
        
        //Used to store output returnred from the channel.
        let mut s = String::new();

        let _ = chan.read_to_string(&mut s)?;

        println!("{}", s); // CONSIDER RETURNING INSTEAD OF PRINTING OUTPUT.

        let _ = chan.wait_close()?;
        
        Ok(())
            
    }
}
