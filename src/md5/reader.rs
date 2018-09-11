use super::types::*;

enum ReaderState {
    DataFlow,
    Padding(u8),
    SizeWriting(u8),
    Ended
}
use self::ReaderState::*;

pub struct Md5Reader<'a> {
    internal: Box<Iterator<Item = &'a u8> + 'a>,
    state: ReaderState,
    len: u64,
}

impl<'a> Md5Reader<'a> {
    pub fn new<I>(base: I) -> Md5Reader<'a> where I: IntoIterator<Item = &'a u8> + 'a {
        Md5Reader{ internal: Box::new(base.into_iter()), state: DataFlow, len: 0 }
    }
}

impl<'a> Iterator for Md5Reader<'a> {
    type Item = WrappingRotating;

    fn next(&mut self) -> Option<WrappingRotating> {
        let mut buf = WrappingRotating(0);

        for _ in 0..3 {
            match self.state {
                DataFlow => {
                    match self.internal.next() {
                        Some(&item) => {
                            self.len += 1;
                            buf = (buf << 8) + item as u32;
                        },
                        None => {
                            self.state = Padding(120 - (self.len % 64) as u8);
                            buf = (buf << 8) + 0x80;
                        }
                    }
                },
                Padding(size) => {
                    buf <<= 8;
                    self.state = if size > 8 { Padding(size - 8) } else { SizeWriting(64) };
                },
                SizeWriting(size) => {
                    buf = (buf << 8) + self.len as u8 as u32;
                    self.len >>= 8;
                    self.state = if size > 8 { SizeWriting(size - 8) } else { Ended }
                },
                Ended => {
                    return None;
                }
            }
        }
        Some(buf)
    }
}
//
//#[cfg(test)]
//mod test {
//
//    #[test]
//    fn test_iterator() {
//        let buf = String::from("12345");
//        let iter = super::Md5Reader::new(buf.as_bytes());
//        let bytes: Vec<u32> = iter.map(|item| item.0).collect();
//        panic!("Was: {:?}, became: {:?}", buf.as_bytes(), bytes);
//    }
//
//}