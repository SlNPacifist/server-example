use std::fs::File;
use std::io::{Read, Write, Bytes, Result, Error, ErrorKind};
use std::str;
use std::thread::sleep;
use std::time::Duration;
use std::char;

const CMD_INIT: &'static str = "AT+CFUN=1\r\n";
const CMD_SET_TEXT_MODE: &'static str = "AT+CMGF=1\r\n";
const CMD_SET_DCS: &'static str = "AT+CSMP=,,0,8\r\n";
const RESPONSE_OK: &'static str = "OK";


fn convert_to_hex(s: &str) -> String {
    let mut res = String::with_capacity(s.len() * 4);
    for c in s.chars() {
        res.push_str(&format!("{:04X}", c as u32));
    }
    res
}

struct Modem {
    reader: Bytes<File>,
    writer: File,
}

impl Modem {
    pub fn create() -> Result<Modem> {
        let mut writer = try!(File::create("/dev/ttyUSB4"));
        let mut reader = try!(File::open("/dev/ttyUSB4")).bytes();
        let mut res = Modem {
            reader: reader,
            writer: writer,
        };
        try!(res.init());
        Ok(res)
    }

    fn init(&mut self) -> Result<()> {
        let res = try!(self.send_command(CMD_INIT));
        if res != RESPONSE_OK {
            Err(Error::new(ErrorKind::Other, "Result for init is not ok"))
        } else {
            Ok(())
        }
    }

    fn send_command(&mut self, cmd: &str) -> Result<String> {
        self.writer.write(cmd.as_bytes());
        self.read_response()
    }

    fn read_line(&mut self) -> Result<String> {
        let mut res = String::new();
        loop {
            match self.reader.next() {
                Some(Err(e)) => return Err(e),
                Some(Ok(code)) => {
                    let symbol = char::from_u32(code as u32)
                        .expect(&format!("Unknown symbol code: {}", code));
                    res.push(symbol);
                    if symbol == '\n' {
                        break;
                    }
                },
                None => {
                    sleep(Duration::new(1, 0));
                }
            }
        }
        Ok(res.trim_right_matches(|c| c == '\n' || c == '\r').to_string())
    }

    fn read_response(&mut self) -> Result<String> {
        self.read_line();
        self.read_line()
    }

    pub fn send_sms(&mut self, number: &str, text: &str) -> Result<()> {
        let res = try!(self.send_command(CMD_SET_TEXT_MODE));
        if res != RESPONSE_OK {
            return Err(Error::new(ErrorKind::Other, "Result for set text mode is not ok"))
        };
        self.send_command(CMD_SET_DCS);
        let message = format!("AT+CMGS=\"{}\"\r\n", number);
        self.writer.write(message.as_bytes());
        self.read_line();
        self.writer.write(convert_to_hex(text).as_bytes());
        self.writer.write("\x1A".as_bytes());
        self.read_line();
        self.read_line();
        self.read_line();
        Ok(())
    }
}


pub fn start_sms_daemon() {
    match Modem::create() {
        Ok(ref mut modem) => {
            println!("Modem created");
            match modem.send_sms("+79027696574", "Тест на русском") {
                Ok(_) => println!("Message sent"),
                Err(err) => println!("Message not sent: {}", err),
            }
        }
        Err(e) => println!("Modem creation failed: {}", e)
    }
}