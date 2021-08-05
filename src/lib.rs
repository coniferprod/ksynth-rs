pub mod k5000;

pub trait SystemExclusiveData {
    fn from_bytes(data: Vec<u8>) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Checksum {
    fn checksum(&self) -> u8;
}

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
