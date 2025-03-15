use crate::bit::Bit;

pub fn to_bits_str(data: &str) -> Vec<Bit> {
    data.chars()
        .map(|c| {
            let mut bits = vec![];
            for i in 0..8 {
                let bit = (c as u8 >> i) & 1;
                bits.push(if bit == 1 {
                    Bit::One(false)
                } else {
                    Bit::Zero(false)
                });
            }
            bits.reverse();
            bits
        })
        .flatten()
        .collect()
}

pub fn to_bits_array(data: &[u8]) -> Vec<Bit> {
    data.iter()
        .map(|c| {
            let mut bits = vec![];
            for i in 0..8 {
                let bit = (c >> i) & 1u8;
                bits.push(if bit == 1u8 {
                    Bit::One(false)
                } else {
                    Bit::Zero(false)
                });
            }
            bits.reverse();
            bits
        })
        .flatten()
        .collect()
}

#[derive(Copy, Clone)]
pub enum Encoding {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

impl Encoding {
    pub fn mod_indicator(&self) -> Vec<Bit> {
        match self {
            Encoding::Numeric => {
                vec![
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::One(false),
                ]
            }
            Encoding::Alphanumeric => {
                vec![
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::One(false),
                    Bit::Zero(false),
                ]
            }
            Encoding::Byte => {
                vec![
                    Bit::Zero(false),
                    Bit::One(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                ]
            }
            Encoding::Kanji => {
                vec![
                    Bit::One(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                ]
            }
        }
    }

    pub fn encode(&self, data: &str) -> Result<Vec<Bit>, String> {
        match self {
            Encoding::Numeric => Encoding::encode_numeric(data),
            Encoding::Alphanumeric => Encoding::encode_alphanumeric(data),
            Encoding::Byte => Encoding::encode_byte(data),
            Encoding::Kanji => {
                unimplemented!()
            }
        }
    }

    fn encode_alphanumeric(data: &str) -> Result<Vec<Bit>, String> {
        let pairs = data
            .chars()
            .map(|c| Self::alphanumeric_value(c))
            .collect::<Result<Vec<u16>, String>>();

        match pairs {
            Ok(vec) => Ok(vec
                .chunks(2)
                .flat_map(|pair| {
                    if pair.len() == 2 {
                        let value = pair[0] * 45 + pair[1];
                        Bit::from(value as u32, 11, false, true)
                    } else {
                        let value = pair[0];
                        Bit::from(value as u32, 6, false, true)
                    }
                })
                .collect()),
            Err(msg) => Err(msg),
        }
    }

    fn alphanumeric_value(c: char) -> Result<u16, String> {
        match c {
            '0'..='9' => Ok(c as u16 - '0' as u16),
            'A'..='Z' => Ok(c as u16 - 'A' as u16 + 10),
            ' ' => Ok(36),
            '$' => Ok(37),
            '%' => Ok(38),
            '*' => Ok(39),
            '+' => Ok(40),
            '-' => Ok(41),
            '.' => Ok(42),
            '/' => Ok(43),
            ':' => Ok(44),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }

    fn encode_byte(data: &str) -> Result<Vec<Bit>, String> {
        let bytes: Result<Vec<u8>, String> =
            data.chars().map(|c| Self::char_to_iso_8859_1(c)).collect();

        match bytes {
            Ok(vec) => Ok(vec
                .iter()
                .flat_map(|byte| Bit::from(*byte as u32, 8, false, true))
                .collect()),
            Err(msg) => Err(msg),
        }
    }

    fn char_to_iso_8859_1(c: char) -> Result<u8, String> {
        if (c as u32) <= 0xFF {
            Ok(c as u8)
        } else {
            Err(format!("Invalid character: {}", c))
        }
    }

    fn encode_numeric(data: &str) -> Result<Vec<Bit>, String> {
        let mut bits = vec![];
        let mut i = 0;
        while i < data.len() {
            let mut value = 0;
            for j in 0..3 {
                if i + j < data.len() {
                    let digit = data.chars().nth(i + j).unwrap();
                    if digit < '0' || digit > '9' {
                        return Err(format!("Invalid character: {}", digit));
                    }
                    value = value * 10 + (digit as u32 - '0' as u32);
                } else {
                    break;
                }
            }
            bits.append(&mut Bit::from(value, 10, false, true));
            i += 3;
        }
        Ok(bits)
    }
}
