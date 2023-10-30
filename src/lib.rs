mod reader;
mod writer;

pub use reader::TapReader;
pub use writer::TapWriter;

#[derive(Debug, PartialEq)]
pub struct BlockInfo {
    pub block_size: usize,
    pub block_number: u32,
    pub is_error: bool,
}

pub(crate) const TAP_FILEMARK: u32 = 0x0000_0000;
pub(crate) const HEADER_MASK: u32 = 0x00FF_FFFF;
pub(crate) const HEADER_EOF: u32 = 0xFFFF_FFFF;

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode() {
        let data = vec![1u8; 20];

        let mut iterator = TapWriter::new(Cursor::new(&data), 16);
        assert_eq!(
            iterator.next(),
            Some((
                vec![16, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 16, 0, 0, 0],
                BlockInfo {
                    block_size: 16,
                    block_number: 0,
                    is_error: false
                }
            ))
        );
        assert_eq!(
            iterator.next(),
            Some((
                vec![4, 0, 0, 0, 1, 1, 1, 1, 4, 0, 0, 0],
                BlockInfo {
                    block_size: 4,
                    block_number: 1,
                    is_error: false
                }
            ))
        );
        assert_eq!(
            iterator.next(),
            Some((
                vec![0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff],
                BlockInfo {
                    block_size: 0,
                    block_number: 2,
                    is_error: false
                }
            ))
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn decode() {
        let data = vec![
            8, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 8, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff,
        ];

        let mut iterator = TapReader::new(Cursor::new(&data));
        assert_eq!(
            iterator.next(),
            Some((
                vec![1u8; 8],
                BlockInfo {
                    block_size: 8,
                    block_number: 0,
                    is_error: false
                }
            ))
        );
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn roundtrip() {
        let data = vec![1u8; 65536 * 10];

        let iterator = TapWriter::new(Cursor::new(&data), 4096);
        let encoded = iterator.map(|(e, _f)| e).flatten().collect::<Vec<_>>();

        let iterator = TapReader::new(Cursor::new(&encoded));
        let decoded = iterator.map(|(d, _f)| d).flatten().collect::<Vec<_>>();

        assert_eq!(data, decoded);
    }
}
