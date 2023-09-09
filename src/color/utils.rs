use crate::error::Error;

pub fn color_string_splitter(color: &String) -> Vec<&str> {
    if color.contains(",") {
        color.trim().split(",").collect::<Vec<&str>>()
    } else {
        color.trim().split_ascii_whitespace().collect::<Vec<&str>>()
    }
}

pub fn safe_value<T>(low: T, high: T, value: T) -> T
where
    T: PartialOrd + Copy,
{
    let min = if low < value { value } else { low };
    if high > min {
        min
    } else {
        high
    }
}

pub fn hex_to_u8(value: String) -> Result<u8, Error> {
    let value = if value.len() == 2 {
        value
    } else if value.len() == 1 {
        value.repeat(2)
    } else {
        return Err(Error::new("Cannot read hex value"));
    };
    let result = hex::decode(value);
    match result {
        Ok(result) => Ok(result[0]),
        Err(_) => Err(Error::new("Cannot decode hex value")),
    }
}
