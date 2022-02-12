
trait DataType<T> {


    fn new(value: T) -> Self;
    fn from_(raw: &[u8]) -> Self;

    fn get(&self) -> T;
    fn parse(raw: &[u8]) -> T;
    fn build(&self) -> Box<[u8]>;
}


pub struct Int {
    value: i32,
}

impl DataType<i32> for Int {
    fn new(value: i32) -> Self {
        Int {value}
    }

    fn from_(raw: &[u8]) -> Int {
        Int {value: Int::parse(raw)}
    }

    fn get(&self) -> i32 {
        self.value.clone()
    }

    fn parse(raw: &[u8]) -> i32 {
        i32::from_ne_bytes(raw.try_into().unwrap())
    }

    fn build(&self) -> Box<[u8]> {
        self.value.to_ne_bytes().into()
    }
}


pub struct Str {
    value: String,
}

impl DataType<String> for Str {
    fn new(value: String) -> Str {
        Str {value}
    }

    fn from_(raw: &[u8]) -> Self {
        Str {value: Self::parse(raw)}
    }

    fn get(&self) -> String {
        self.value.clone()
    }

    fn parse(raw: &[u8]) -> String {
        String::from_utf8(raw.into()).unwrap_or(String::from("#!ERR"))
    }

    fn build(&self) -> Box<[u8]> {
        self.value.clone().into_bytes().into()
    }
}



#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;

    #[test]
    fn test_int() {
        let i = Int::new(5);

        assert_eq!(i.get(), 5);
        if cfg!(target_endian = "big") {
            assert_eq!(Int::parse(&[0u8, 0u8, 0u8, 5u8]), 5);
            assert_eq!(Int::from_(&[0u8, 0u8, 0u8, 5u8]).get(), 5);
            assert_eq!(i.build().deref(), [0u8, 0u8, 0u8, 5u8]);
        } else {
            assert_eq!(Int::parse(&[5u8, 0u8, 0u8, 0u8]), 5);
            assert_eq!(Int::from_(&[5u8, 0u8, 0u8, 0u8]).get(), 5);
            assert_eq!(i.build().deref(), [5u8, 0u8, 0u8, 0u8]);
        }
    }

    #[test]
    fn test_str() {
        let text: String = "Amour Plastique".into();
        let byte_array = [65, 109, 111, 117, 114, 32, 80, 108, 97, 115, 116, 105, 113, 117, 101];

        let i = Str::new(text.clone());
        assert_eq!(i.get(), text.clone());
        assert_eq!(i.build().deref(), byte_array.clone());
        assert_eq!(Str::parse(&byte_array), text.clone());
        assert_eq!(Str::from_(&byte_array).get(), text.clone());
    }

}