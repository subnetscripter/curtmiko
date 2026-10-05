#![allow(warnings)]

use std::error::Error;
use telnet::{Event, Telnet};
use std::time::Duration;
use regex::Regex;

fn main() {
    
    let mut node = Node::build("10.0.0.66", 23).unwrap();
    println!("Telnet connection to node succesful!");

    let mut reg = Regex::new(r":$").unwrap();
    let _ = node.authenticate("george", "george@123", &reg);
    println!("Authenticaton to node successful");

    let mut reg = Regex::new(r"#$").unwrap();
    node.send_command("conf t", &reg);
    node.send_command("username timothy password Grimmjow@1234", &reg);
    node.send_command("username Aimothy password Grimmjow@1234", &reg);
    node.send_command("username Cimothy password Grimmjow@1234", &reg);
    

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


    // #BUG DETECTED#
    //If the user credentials do not exist on the remote node
    //The terminal hangs. Need to create error handling for this situation.
    pub fn authenticate(&mut self, username: &str, password: &str, reg: &Regex) -> Result<(), Box<dyn Error>> {
        self.read_data(&reg);
        let _ = self.write_data(username)?;

        self.read_data(&reg);
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
                        //println!("{hay_buffer}"); // THIS LINE TO BE REMOVED
                        break;
                    },
                    None => continue
                };
            };
        }
    }

    //Used to send data to the connection/
    //A new line is always sent at the end.
    fn write_data(&mut self, string_data: &str) -> Result<(), Box<dyn Error>> {
        let data = string_data.as_bytes();
        let mut result = self.conn.write(data)?;
        result = self.conn.write(b"\n")?;
        Ok(())

    }

    pub fn send_command(&mut self, command: &str, reg: &Regex) -> Result<(), Box<dyn Error>> {
        //Looks for the correct ending prompt to ensure that you can send the required configs.
        self.read_data(reg);
        match self.write_data(command) {
            Ok(t) => println!("Sent command: {command}"),
            Err(e) => return Err(e)
        }
        Ok(())
    }

}




