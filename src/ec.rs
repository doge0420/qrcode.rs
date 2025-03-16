use crate::tables::{DATA_BYTES_PER_BLOCK, EXP_TABLE, GENERATOR_POLYNOMIALS, LOG_TABLE};

#[derive(Clone, Copy)]
pub enum EcLevel {
    H,
    Q,
    M,
    L,
}

impl EcLevel {
    pub fn ordinal(&self) -> u8 {
        match self {
            EcLevel::L => 0,
            EcLevel::M => 1,
            EcLevel::Q => 2,
            EcLevel::H => 3,
        }
    }
}

pub fn error_correction(
    data: &Vec<u8>,
    version: u8,
    ec_level: &EcLevel,
    cw_per_block: usize,
) -> Vec<u8> {
    let blocks = groups(data, version, ec_level);

    let ec_blocks = blocks
        .iter()
        .map(|block| {
            create_ec_for_block(
                Vec::from(block.clone()),
                cw_per_block,
                GENERATOR_POLYNOMIALS[cw_per_block],
            )
        })
        .collect::<Vec<Vec<u8>>>();

    interleave(ec_blocks)
}

pub fn groups(data: &Vec<u8>, version: u8, ec_level: &EcLevel) -> Vec<Vec<u8>> {
    let ec_level = ec_level.ordinal();
    let (block_1_size, block_1_count, block_2_size, block_2_count) =
        DATA_BYTES_PER_BLOCK[(version - 1) as usize][ec_level as usize];

    let group_1_size = block_1_count * block_1_size;

    let mut blocks = Vec::with_capacity(block_1_count + block_2_count);

    if group_1_size < data.len() {
        let (group_1, group_2) = data.split_at(group_1_size);

        group_1.chunks(block_1_size).for_each(|block| {
            blocks.push(block.to_vec());
        });
        if block_2_size > 0 {
            group_2.chunks(block_2_size).for_each(|block| {
                blocks.push(block.to_vec());
            });
        }
    } else {
        data.chunks(block_1_size).for_each(|block| {
            blocks.push(block.to_vec());
        });
    }

    blocks
}

fn create_ec_for_block(block: Vec<u8>, ec_size: usize, generator_polynomial: &[u8]) -> Vec<u8> {
    let data_len = block.len();
    let mut codewords = block;
    codewords.resize(data_len + ec_size, 0);

    for i in 0..data_len {
        let lead_coeff = codewords[i];
        if lead_coeff == 0 {
            continue;
        }
        let log_lead_coeff = usize::from(LOG_TABLE[lead_coeff as usize]);

        codewords[i + 1..]
            .iter_mut()
            .zip(generator_polynomial.iter())
            .for_each(|(cw, &gen_coeff)| {
                *cw ^= EXP_TABLE[(usize::from(gen_coeff) + log_lead_coeff) % 255];
            });
    }

    codewords.split_off(data_len)
}

pub fn interleave(blocks: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = Vec::new();
    let max_len = blocks.iter().map(|block| block.len()).max().unwrap();
    for i in 0..max_len {
        for block in &blocks {
            if i < block.len() {
                result.push(block[i]);
            }
        }
    }
    result
}

#[cfg(test)]
mod interleave_tests {
    use super::*;

    #[test]
    fn interleave_works_with_equal_length_blocks() {
        let blocks = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = interleave(blocks);
        assert_eq!(result, vec![1, 4, 7, 2, 5, 8, 3, 6, 9]);
    }

    #[test]
    fn interleave_works_with_unequal_length_blocks() {
        let blocks = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
        let result = interleave(blocks);
        assert_eq!(result, vec![1, 3, 6, 2, 4, 5]);
    }

    #[test]
    fn interleave_works_with_empty_blocks() {
        let blocks: Vec<Vec<u8>> = vec![vec![], vec![], vec![]];
        let result = interleave(blocks);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn interleave_works_with_single_block() {
        let blocks = vec![vec![1, 2, 3]];
        let result = interleave(blocks);
        assert_eq!(result, vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod ec_tests {
    use super::*;

    #[test]
    fn create_ec_for_block_works_simple() {
        let block = vec![1, 2, 3];
        let ec_size = block.len();
        let generator_polynomial = GENERATOR_POLYNOMIALS[ec_size];

        let ec = create_ec_for_block(block, ec_size, generator_polynomial);
        assert!(ec.eq(&vec![92, 236, 176]));
    }

    #[test]
    fn create_ec_for_block_works_complex() {
        let block = vec![32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236];
        let ec_size = block.len();
        let generator_polynomial = GENERATOR_POLYNOMIALS[ec_size];

        let ec = create_ec_for_block(block, ec_size, generator_polynomial);
        assert!(ec.eq(&vec![
            168, 72, 22, 82, 217, 54, 156, 0, 46, 15, 180, 122, 16
        ]));
    }
}
