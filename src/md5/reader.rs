use super::types::*;

#[derive(Debug)]
enum ReaderState {
    DataFlow,
    Padding(u8),
    SizeWriting(u8),
    Ended
}
use self::ReaderState::*;

pub struct Md5Reader<'a> {
    internal: Box<Iterator<Item = u8> + 'a>,
    state: ReaderState,
    len: u64,
}

impl<'a> Md5Reader<'a> {
    pub fn new<I>(base: I) -> Md5Reader<'a> where I: IntoIterator<Item = u8> + 'a {
        Md5Reader{ internal: Box::new(base.into_iter()), state: DataFlow, len: 0 }
    }
}

impl<'a> Iterator for Md5Reader<'a> {
    type Item = [WrappingRotating; 16];

    fn next(&mut self) -> Option<[WrappingRotating; 16]> {
        let mut list = [WrappingRotating(0); 16];

        for index in 0..=15 {
            println!("Generating element {}", index);
            for _ in 0..=3 {
                println!("Reader state: {:?}", self.state);
                match self.state {
                    DataFlow => {
                        match self.internal.next() {
                            Some(item) => {
                                self.len += 1;
                                println!("Read item: {}, current length: {}", item, self.len);
                                list[index] = (list[index] << 8) + item as u32;
                            },
                            None => {
                                self.state = Padding(64 - ((self.len + 9) % 64) as u8);
                                list[index] = (list[index] << 8) + 0x80;
                            }
                        }
                    },
                    Padding(size) => {
                        list[index] <<= 8;
                        self.state = if size > 1 { Padding(size - 1) } else { SizeWriting(8) };
                    },
                    SizeWriting(size) => {
                        list[index] = (list[index] << 8) + self.len as u8 as u32;
                        self.len >>= 8;
                        self.state = if size > 1 { SizeWriting(size - 1) } else { Ended }
                    },
                    Ended => {
                        return None;
                    }
                }
            }
        }
        Some(list)
    }
}

//#[cfg(test)]
//mod test {
//
//    #[test]
//    fn test_iterator() {
//        let buf = "";
//        let iter = super::Md5Reader::new(buf.as_bytes());
//        let bytes: Vec<u32> = iter.map(|item| item[0]).collect();
//        panic!("Was: {:?}, became: {:?}", buf.as_bytes(), bytes);
//    }
//
//}