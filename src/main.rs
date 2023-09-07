use std::io::{self, Read, stdout};
use termion::raw::IntoRawMode;

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;

    // 'ctrl + key' bytes only contain 5 bits
    byte & 0b0001_1111
}

fn die(err: std::io::Error) {
    panic!(e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for byte in io::stdin().bytes() {
        
        match byte {
            Ok(byte) => {
                println!("{:#b} \r", byte);

                let character = byte as char;

                if character.is_control() {
                    println!("{:?} \r", byte);
                } else {
                    println!("{:?} ({})\r", byte, character);
                }

                if byte == to_ctrl_byte('q') {
                    break;
                }
            },
            Err(err) => die(err),
        }
        
    }
}
