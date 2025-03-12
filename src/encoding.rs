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
                    Bit::One(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                ]
            }
            Encoding::Alphanumeric => {
                vec![
                    Bit::Zero(false),
                    Bit::One(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                ]
            }
            Encoding::Byte => {
                vec![
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::One(false),
                    Bit::Zero(false),
                ]
            }
            Encoding::Kanji => {
                vec![
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::Zero(false),
                    Bit::One(false),
                ]
            }
        }
    }

    pub fn encode(&self, data: &str) -> Result<Vec<Bit>, String> {
        match self {
            Encoding::Numeric => {
                unimplemented!()
            }
            Encoding::Alphanumeric => Encoding::encode_alphanumeric(data),
            Encoding::Byte => {
                unimplemented!()
            }
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
                        Bit::from(value as u32, 11, false, false)
                    } else {
                        let value = pair[0];
                        Bit::from(value as u32, 6, false, false)
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
}
