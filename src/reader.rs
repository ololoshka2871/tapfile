use std::io::Read;

use crate::{BlockInfo, HEADER_EOF, HEADER_MASK, TAP_FILEMARK};

pub struct TapReader<T: Read> {
    reader: T,
    block_count: u32,
}

impl<T: Read> TapReader<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            block_count: 0,
        }
    }
}

impl<T: Read> Iterator for TapReader<T> {
    type Item = (Vec<u8>, BlockInfo);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 4];

        match self.reader.read(&mut buf) {
            Ok(sz) => {
                if sz != std::mem::size_of::<u32>() {
                    return None;
                }
            }
            Err(_e) => return None,
        };

        let header1 = u32::from_le_bytes(buf);

        if header1 == HEADER_EOF || header1 == TAP_FILEMARK {
            // end or new file (not supported)
            return None;
        }

        let block_size = header1 & HEADER_MASK;
        let is_error = (header1 & (!HEADER_MASK)) != 0;

        let mut data_buf = vec![0u8; block_size as usize];
        let res = match self.reader.read(&mut data_buf) {
            Ok(sz) => {
                if sz != block_size as usize {
                    return None;
                } else {
                    let info = BlockInfo {
                        block_size: block_size as usize,
                        block_number: self.block_count,
                        is_error,
                    };
                    self.block_count += 1;

                    Some((data_buf, info))
                }
            }
            Err(_e) => return None,
        };

        match self.reader.read(&mut buf) {
            Ok(sz) => {
                if sz != std::mem::size_of::<u32>() {
                    return None;
                }
            }
            Err(_e) => return None,
        };

        let header2 = u32::from_le_bytes(buf);

        if header1 != header2 {
            None
        } else {
            res
        }
    }
}
