use crate::bit::Bit;

pub fn to_bits(data: &str) -> Vec<Bit> {
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

pub fn mod_indicator(encoding: &Encoding) -> Vec<Bit> {
    match encoding {
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

#[derive(Copy, Clone)]
pub enum Encoding {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}
