use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct ZeusHeader {
    pub magic: [u8; 4],
    pub version: [u8; 3],
    pub zero: [u8; 25]
}

pub struct Rom {
    pub header: ZeusHeader,
    pub data: Vec<u8>
}

impl Rom {
    pub fn load(path: &String) -> Result<Rom, io::Error> {
        let mut f = File::open(path).expect("Rom file not found");
        let mut header_buffer = [0;32];
        f.read_exact(&mut header_buffer)?;

        let header = ZeusHeader {
            magic: [
                header_buffer[0],
                header_buffer[1],
                header_buffer[2],
                header_buffer[3]
            ],
            version: [
                header_buffer[4],
                header_buffer[5],
                header_buffer[6]
            ],
            zero: [0; 25]
        };

        if header.magic != *b"ZEUS" {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Loaded file is not a Zeus rom."));
        }

        let mut data = vec!();
        f.read_to_end(&mut data)?;

        Ok(Rom { header, data })
    }
}
