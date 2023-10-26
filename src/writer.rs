use std::io::{Read, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{BlockInfo, HEADER_EOF, HEADER_MASK};

pub struct TapWriter<T> {
    input: T,
    block_size: usize,
    current_block: u32,
    end_marker: bool,
}

impl<T: Read> TapWriter<T> {
    pub fn new(input: T, block_size: usize) -> Self {
        Self {
            input,
            block_size: block_size,
            current_block: 0,
            end_marker: false,
        }
    }
}

impl<T: Read> Iterator for TapWriter<T> {
    type Item = (Vec<u8>, BlockInfo);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_marker {
            return None;
        }

        let mut buf = vec![0; self.block_size];

        match self.input.read(&mut buf) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    let eof = HEADER_EOF.to_le_bytes();
                    let d = (0..2).map(|_| eof).flatten().collect::<Vec<_>>();
                    self.end_marker = true;
                    Some((
                        d,
                        BlockInfo {
                            block_size: 0,
                            block_number: self.current_block,
                            is_error: false,
                        },
                    ))
                } else {
                    let header = (bytes_read as u32) & HEADER_MASK;
                    let info = BlockInfo {
                        block_size: bytes_read,
                        block_number: self.current_block,
                        is_error: false,
                    };
                    let mut output_cursor =
                        std::io::Cursor::new(vec![
                            0u8;
                            std::mem::size_of_val(&header) * 2 + bytes_read
                        ]);
                    output_cursor.write_u32::<LittleEndian>(header).unwrap();
                    output_cursor.write_u32::<LittleEndian>(header).unwrap();
                    output_cursor.write_all(&buf[..bytes_read]).unwrap();
                    self.current_block += 1;

                    Some((output_cursor.into_inner(), info))
                }
            }
            Err(_e) => None,
        }
    }
}
