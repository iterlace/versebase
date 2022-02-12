use chrono;
use chrono::NaiveDateTime;


trait DataType<T> {
    fn new(value: T) -> Self;
    // TODO: rename to "from"
    fn from_(raw: &[u8]) -> Self;
    fn deserialize(raw: &[u8]) -> T;

    fn get(&self) -> T;
    fn serialize(&self) -> Box<[u8]>;
}


pub struct Int {
    value: i32,
}

impl DataType<i32> for Int {
    fn new(value: i32) -> Self {
        Self {value}
    }

    fn from_(raw: &[u8]) -> Int {
        Self {value: Int::deserialize(raw)}
    }

    fn deserialize(raw: &[u8]) -> i32 {
        i32::from_ne_bytes(raw.try_into().unwrap_or([0, 0, 0, 0]))
    }

    fn get(&self) -> i32 {
        self.value.clone()
    }

    fn serialize(&self) -> Box<[u8]> {
        self.value.to_ne_bytes().into()
    }
}


pub struct Str {
    value: String,
}

impl DataType<String> for Str {
    fn new(value: String) -> Str {
        Self {value}
    }

    fn from_(raw: &[u8]) -> Self {
        Self {value: Self::deserialize(raw)}
    }

    fn deserialize(raw: &[u8]) -> String {
        String::from_utf8(raw.into()).unwrap_or(String::from("#!ERR"))
    }

    fn get(&self) -> String {
        self.value.clone()
    }

    fn serialize(&self) -> Box<[u8]> {
        self.value.clone().into_bytes().into()
    }
}


pub struct DateTime {
    value: chrono::NaiveDateTime,
}

impl DataType<chrono::NaiveDateTime> for DateTime {
    fn new(value: chrono::NaiveDateTime) -> Self {
        Self {value}
    }

    fn from_(raw: &[u8]) -> Self {
        Self {value: Self::deserialize(raw)}
    }

    fn deserialize(raw: &[u8]) -> chrono::NaiveDateTime {
        let raw_: i64 = i64::from_ne_bytes(
            raw.try_into().unwrap_or([0, 0, 0, 0, 0, 0, 0, 0])
        );
        chrono::NaiveDateTime::from_timestamp(
            raw_ / 1_000_000_000,
            (raw_  % 1_000_000_000) as u32,
        )
    }

    fn get(&self) -> chrono::NaiveDateTime {
        self.value.clone()
    }

    fn serialize(&self) -> Box<[u8]> {
        self.value.timestamp_nanos().to_ne_bytes().into()
    }
}



#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;

    #[test]
    fn test_int() {
        let num = 5;
        let byte_array_be = [0u8, 0u8, 0u8, 5u8];
        let byte_array_le = [5u8, 0u8, 0u8, 0u8];

        let obj = Int::new(num);

        assert_eq!(obj.get(), num);

        if cfg!(target_endian = "big") {
            assert_eq!(Int::deserialize(&byte_array_be), num);
            assert_eq!(Int::from_(&byte_array_be).get(), num);
            assert_eq!(obj.serialize().deref(), byte_array_be);
        } else {
            assert_eq!(Int::deserialize(&byte_array_le), num);
            assert_eq!(Int::from_(&byte_array_le).get(), num);
            assert_eq!(obj.serialize().deref(), byte_array_le);
        }
    }

    #[test]
    fn test_str() {
        let text: String = "Amour Plastique".into();
        let byte_array = [65, 109, 111, 117, 114, 32, 80, 108, 97, 115, 116, 105, 113, 117, 101];

        let obj = Str::new(text.clone());
        assert_eq!(obj.get(), text.clone());
        assert_eq!(obj.serialize().deref(), byte_array.clone());
        assert_eq!(Str::deserialize(&byte_array), text.clone());
        assert_eq!(Str::from_(&byte_array).get(), text.clone());
    }

    #[test]
    fn test_datetime() {
        let datetime = chrono::NaiveDateTime::from_timestamp(60, 1024);
        let byte_array_be = [0, 0, 0, 13, 248, 71, 92, 0];
        let byte_array_le = [0, 92, 71, 248, 13, 0, 0, 0];

        let obj = DateTime::new(datetime.clone());

        assert_eq!(obj.get(), datetime);

        if cfg!(target_endian = "big") {
            assert_eq!(DateTime::deserialize(&byte_array_be), datetime);
            assert_eq!(DateTime::from_(&byte_array_be).get(), datetime);
            assert_eq!(obj.serialize().deref(), byte_array_be);
        } else {
            assert_eq!(DateTime::deserialize(&byte_array_le), datetime);
            assert_eq!(DateTime::from_(&byte_array_le).get(), datetime);
            assert_eq!(obj.serialize().deref(), byte_array_le);
        }
    }

}