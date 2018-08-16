use std::iter;

struct Md5Reader<T> {
    internal: Box<Iterator<Item = T>>,
    ending: bool,
    len: u64
}

impl Md5Reader<u8> {
    fn new<I>(iter: I) -> Md5Reader<u8> where I: Iterator<Item = u8> {
        Md5Reader{internal: Box::new(iter), ending: false, len: 0}
    }
}

impl Iterator for Md5Reader<u8> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match self.internal.next() {  
            Some(item) => {
                self.len += 1;
                Some(item)
            },
            None => match self.ending {
                true => None,
                false => {
                    self.ending = true;
                    self.internal = ending_generate(self.len);
                    self.internal.next()
                }
            }
        }
    }
}

fn ending_generate(len: u64) -> Box<Iterator<Item = u8>> {
    let vec = vec![0x80];

    // Number of additional bytes needed to make the len (in bits) respect:
    // (vec.len() * 8) == 448 (mod 512) i.e. vec.len() == 56 (mod 64)
    let mut diff = 56 - (vec.len() % 64) as i8;

    // We can't pad a negative number of bits, so pad up to the next multiple
    // of 64.
    if diff < 0 {
        diff += 64;
    }
    vec.extend(iter::repeat(0).take(diff as usize));

    // adding the length, as stated, from the least-significant byte
    for shift in 0..7 {
        vec.push((len >> (shift * 8)) as u8);
    }

    assert_eq!(vec.len() % 64, 0);
    Box::new(vec.into_iter())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_iterator() {
        let iter = Md5Reader::new(String::from("12345").as_bytes().into_iter());
        let bytes: Vec<u8> = iter.collect();
        fail!(bytes);
    }
}