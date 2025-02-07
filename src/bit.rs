#[derive(Copy, Clone, Debug)]
pub enum Bit {
    One(bool),
    Zero(bool),
}

impl PartialEq for Bit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bit::One(val1), Bit::One(val2)) => val1 == val2,
            (Bit::Zero(val1), Bit::Zero(val2)) => val1 == val2,
            (_, _) => false,
        }
    }
}

impl Bit {
    pub fn is_functional(&self) -> bool {
        match self {
            Bit::One(val) => *val,
            Bit::Zero(val) => *val,
        }
    }

    pub fn value(&self) -> bool {
        match self {
            Bit::One(_) => true,
            Bit::Zero(_) => false,
        }
    }

    pub fn from(data: u32, n_bits: u8, functional: bool, reverse: bool) -> Vec<Bit> {
        let mut bits = Vec::new();
        for i in 0..n_bits {
            let mask = if reverse {
                1 << (n_bits - i - 1)
            } else {
                1 << i
            };
            let bit = (data & mask) != 0;
            match bit {
                true => {
                    bits.push(Bit::One(functional));
                }
                false => {
                    bits.push(Bit::Zero(functional));
                }
            }
        }
        bits
    }
}
