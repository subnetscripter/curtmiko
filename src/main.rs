#![allow(warnings)]

use std::error::Error;
use telnet::{Event, Telnet};
use std::time::Duration;
use regex::Regex;

fn main() {
    
    let mut node = Node::build("10.0.0.66", 23).unwrap();
    println!("Telnet connection to node succesful!");
    let _ = node.authenticate("george", "george@123");
    println!("Authenticaton to node successful");

}


//Node will be a generic struct type that connects to any device
//Right now it only connects via telnet.
//Need to expand to ssh for secure connections in the future.
pub struct Node{
    conn: Telnet,
}

impl Node{
    pub fn build(ip: &str, port: u16) -> Result<Node, Box<dyn Error>> {
        
        let conn = Telnet::connect((ip, port), 1024)?;

        Ok(Node{conn})
    }


    
    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        
        let mut reg = Regex::new(r":$").unwrap();

        read_data(&mut self.conn, &reg);
        let _ = self.write_data(username)?;

        read_data(&mut self.conn, &reg);
        let _ = self.write_data(password)?;

        Ok(())

    }

    //Streams the output from the telnet connections until
    // the desired Regex pattern is matched.
    // I'm considering adding a bool as a parameter in the future
    // that will indicate whether to discard the output or not.
    fn read_data(&mut self, reg: &Regex) {
        
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
                        println!("{hay_buffer}"); // THIS LINE TO BE REMOVED
                        break;
                    },
                    None => continue
                };
            };
        }
    }

    fn write_data(&mut self, string_data: &str) -> Result<(), Box<dyn Error>> {
        let data = string_data.as_bytes();
        let mut result = self.conn.write(data)?;
        result = self.conn.write(b"\n")?;
        Ok(())

    }

}


//Streams the output from the telnet connections until
// the desired Regex pattern is matched.
// I'm considering adding a bool as a parameter in the future
// that will indicate whether to discard the output or not.
fn read_data(conn: &mut Telnet, reg: &Regex) {
    
    //Used to buffer the output from reading the telnet stream.
    //Used for regex search
    let mut hay_buffer = String::new();

    //Loops over telnet Event stream until the Event variant
    // we want (Even::Data)) is found
    loop{
        let event = conn.read_timeout(Duration::from_secs(2));
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


