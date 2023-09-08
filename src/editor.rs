use std::{io::{self, stdout}, default};
use termion::{raw::IntoRawMode, event::Key, input::TermRead};

fn die(err: std::io::Error) {
    panic!("{}", err);
}

pub struct Editor {

}

impl Editor {
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        
                        if c.is_control() {
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }

                    },
                    Key::Ctrl('q') => break,
                    _ => println!("{:?} \r", key),
                },
                Err(err) => die(err),
            }
            
        }
    }

    pub fn default() -> Self {

        Editor {  }
    }
}