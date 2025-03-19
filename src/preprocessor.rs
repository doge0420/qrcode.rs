use crate::bit::Bit;
use crate::debug_vec;
use crate::ec::*;
use crate::encoding::*;
use crate::mask::MaskPattern;
use crate::qrcode::QrCode;
use crate::tables::{
    ALPHANUMERIC_CHAR_COUNT, ALPHANUMERIC_SIZE, BYTE_CHAR_COUNT, BYTE_SIZE, DATA_BYTES_PER_BLOCK,
    EC_BYTES_PER_BLOCK, KANJI_CHAR_COUNT, KANJI_SIZE, NUMERIC_CHAR_COUNT, NUMERIC_SIZE,
};

pub struct Preprocessor {
    qrcode_bits: Vec<Bit>,
    encoding: Encoding,
    ec_level: EcLevel,
    version: u8,
    mask_pattern: MaskPattern,
}

impl Preprocessor {
    pub fn generate_qrcode(&self) -> QrCode {
        let mut res = QrCode::new(
            self.version,
            self.ec_level,
            self.mask_pattern,
            self.encoding,
        )
        .expect("QR code generation error");

        res.all_functional_patterns();
        res.fill(&self.qrcode_bits);
        res.apply_mask();

        res
    }

    pub fn new(
        data: &str,
        encoding: Encoding,
        ec_level: EcLevel,
        mask_pattern: MaskPattern,
    ) -> Preprocessor {
        let mut bits = encoding.encode(data).unwrap();

        bits.iter().enumerate().for_each(|(i, bit)| {
            if bit.value() {
                print!("1")
            } else {
                print!("0")
            }

            if (i + 1) % 8 == 0 {
                print!(" ");
            }
        });
        println!();

        let table = Self::table_from_encoding(encoding);

        let (v, _) = table
            .iter()
            .skip(ec_level.ordinal() as usize)
            .step_by(4)
            .enumerate()
            .find(|(_, &size)| data.len() <= size as usize)
            .expect("Not enough space.");

        let version = v + 1;
        let char_count = Self::char_count(version as u8, encoding);
        let mut char_count_to_bits = Bit::from(data.len() as u32, char_count, false, true);

        if bits.len() < char_count as usize {
            bits.append(&mut vec![
                Bit::Zero(false);
                (char_count as usize) - bits.len()
            ]);
        }

        let mut qrcode_bits = vec![];
        let mut mod_indicator_bits = encoding.mod_indicator();

        // let bytes = Bit::bytes(&bits);
        // let interleaved_bytes = interleave(groups(&bytes, version as u8, &ec_level));
        // let mut interleaved_bits = Bit::bits(&interleaved_bytes, bits.len());

        qrcode_bits.append(&mut mod_indicator_bits);
        qrcode_bits.append(&mut char_count_to_bits);
        qrcode_bits.append(&mut bits);

        // Compute total size without ec bits
        let (block_1_size, block_1_count, block_2_size, block_2_count) =
            DATA_BYTES_PER_BLOCK[version - 1][ec_level.ordinal() as usize];
        let total_data_bits = (block_1_size * block_1_count + block_2_size * block_2_count) * 8;

        // Add terminator bits (at most 4 0s)
        if qrcode_bits.len() < total_data_bits {
            let empty_bits = total_data_bits - qrcode_bits.len();
            if empty_bits >= 4 {
                qrcode_bits.append(&mut vec![Bit::Zero(false); 4]);
            } else {
                qrcode_bits.append(&mut vec![Bit::Zero(false); empty_bits]);
            }
        }

        // Add padding bits
        while qrcode_bits.len() % 8 != 0 {
            qrcode_bits.push(Bit::Zero(false));
        }

        // Add padding bytes
        if qrcode_bits.len() < total_data_bits {
            let mut byte_1 = to_bits_array(&[236]);
            let mut byte_2 = to_bits_array(&[17]);

            while qrcode_bits.len() < total_data_bits {
                qrcode_bits.extend_from_slice(&byte_1);
                std::mem::swap(&mut byte_1, &mut byte_2);
            }
        }

        let cw_per_block = EC_BYTES_PER_BLOCK[version - 1][ec_level.ordinal() as usize];
        let (_block_1_size, block_1_count, _block_2_size, block_2_count) =
            DATA_BYTES_PER_BLOCK[version - 1][ec_level.ordinal() as usize];

        let ec_size = (block_1_count + block_2_count) * cw_per_block;

        // todo!("compute the correct lenght for the number of error correction bits");

        let codewords = codewords(
            &Bit::bytes(&qrcode_bits),
            version as u8,
            &ec_level,
            cw_per_block,
        );

        let mut data_bits = Bit::bits(&codewords.0, codewords.0.len() * 8);
        let mut error_correction = Bit::bits(&codewords.1, ec_size * 8);

        qrcode_bits.append(&mut data_bits);
        qrcode_bits.append(&mut error_correction);

        debug_vec!(&qrcode_bits);

        Preprocessor {
            qrcode_bits,
            encoding,
            ec_level,
            version: version as u8,
            mask_pattern,
        }
    }

    fn table_from_encoding(encoding: Encoding) -> &'static [u32; 160] {
        match encoding {
            Encoding::Numeric => &NUMERIC_SIZE,
            Encoding::Alphanumeric => &ALPHANUMERIC_SIZE,
            Encoding::Byte => &BYTE_SIZE,
            Encoding::Kanji => &KANJI_SIZE,
        }
    }

    fn char_count(version: u8, encoding: Encoding) -> u8 {
        let index = match version {
            1_u8..=9_u8 => 0,
            10_u8..=26_u8 => 1,
            27u8..=40u8 => 2,
            _ => panic!("Invalid version."),
        };
        match encoding {
            Encoding::Numeric => NUMERIC_CHAR_COUNT[index],
            Encoding::Alphanumeric => ALPHANUMERIC_CHAR_COUNT[index],
            Encoding::Byte => BYTE_CHAR_COUNT[index],
            Encoding::Kanji => KANJI_CHAR_COUNT[index],
        }
    }
}
