#![allow(warnings)]

use std::error::Error;
use telnet::{Event, Telnet};
use std::io;
use std::time::Duration;
use regex::Regex;

fn main() {
    
    let mut conn = match Telnet::connect(("10.0.0.66", 23), 1024) {
        Ok(t) => {
            println!("Connected to device successfully!");
            t
        },
        Err(e) => panic!("Could not connect to device!")
    };


    
    let _ = authenticate("george","george@123", &mut conn);
    //let _ = write_data("conf t", &mut conn).expect("Failed to write data: {e}");
    //let _ = write_data("username applebanana password applebanana@123", &mut conn).expect("Failed to write data: {e}");


}

fn write_data(string_data: &str,  conn: &mut Telnet) -> Result<(), Box<dyn Error>> {
    let data = string_data.as_bytes();
    let mut result = conn.write(data)?;
    result = conn.write(b"\n")?;
    Ok(())

}

//Reads data from a telnet::Telnet connection.
//Change in the future to accomodate telnet and ssh2.
//For now it just prints out the data
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
                    println!("{hay_buffer}");
                    break;
                },
                None => continue
            };
        };
    }
}


fn authenticate(username: &str, password: &str, conn: &mut Telnet) -> Result<(), Box<dyn Error>> {
    
    let mut reg = Regex::new(r":$").unwrap();

    read_data(conn, &reg);
    let _ = write_data(username, conn)?;

    read_data(conn, &reg);
    let _ = write_data(password, conn)?;

    Ok(())

}
