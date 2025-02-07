use crate::bit::Bit;

pub fn to_bits(data: String) -> Vec<Bit> {
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

pub fn with_mod_indicator(data: &mut Vec<Bit>, encoding: Encoding) {
    let mode = match encoding {
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
    };

    data.splice(0..0, mode);
}

pub enum Encoding {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}
