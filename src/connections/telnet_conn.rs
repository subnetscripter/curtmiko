use telnet::{Telnet, Event};
use regex::Regex;
use std::error::Error;
use std::time::Duration;

/*

COMPLETED
*/

pub struct TelnetConn{
    conn: Telnet,
}

impl TelnetConn{
    pub fn build(ip: &str, port: u16) -> Result<TelnetConn, Box<dyn Error>> {

        //CONSIDER MAKING BUFFER SIZE A PASSABLE VALUE
        //INSTEAD OF HARD CODING 1024.
        let conn = Telnet::connect((ip, port), 1024)?;

        Ok(TelnetConn{conn})
    }


    //Used to send data to the connection/
    //A new line is always sent at the end.
    pub fn write_data(&mut self, string_data: &str) -> Result<(), Box<dyn Error>> {
        let data = string_data.as_bytes();
        let mut result = self.conn.write(data)?;
        result = self.conn.write(b"\n")?;
        Ok(())

    }

    //Streams the output from the telnet connections until
    // the desired Regex pattern is matched.
    pub fn read_data(&mut self, reg: &Regex) {
        
        //Used to buffer the output from reading the telnet stream.
        //Used for regex search
        let mut hay_buffer = String::new();

        //Loops over telnet Event stream until the Event variant
        // we want (Even::Data)) is found
        loop{
            let event = self.conn.read_timeout(Duration::from_secs(2));
            if let Ok(Event::Data(data)) = event{
                let data_str = String::from_utf8_lossy(&data);
                hay_buffer.push_str(data_str[..].trim());
                match reg.find(&hay_buffer[..]){
                    Some(t) => {
                        //println!("{hay_buffer}"); // THIS LINE TO BE REMOVED
                        break;
                    },
                    None => continue
                };
            };
        }
    }
    
}

