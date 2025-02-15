use crate::bit::Bit;

impl Bit {
    pub fn bytes(data: &Vec<Bit>) -> Vec<u8> {
        data.chunks(8)
            .map(|chunk| {
                let mut byte = 0u8;
                for (i, bit) in chunk.iter().enumerate() {
                    byte |= (bit.value() as u8) << (7 - i);
                }
                byte
            })
            .collect()
    }

    pub fn bits(data: &Vec<u8>) -> Vec<Bit> {
        data.iter()
            .map(|byte| {
                (0..8)
                    .map(|i| {
                        let bit = (byte >> (7 - i)) & 1u8;
                        match bit {
                            1 => Bit::One(false),
                            0 => Bit::Zero(false),
                            _ => unreachable!(),
                        }
                    })
                    .collect::<Vec<Bit>>()
            })
            .flatten()
            .collect()
    }
}
