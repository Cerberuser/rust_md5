struct Md5Reader<T> {
    internal: Box<Iterator<Item = T>>
}

impl Iterator for Md5Reader<u32> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.internal.next() {  
            Some(item) => item
            None => None
        }
    }
}