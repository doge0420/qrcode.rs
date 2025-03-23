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

        let mut data_segment = Vec::new();
        let mut mod_indicator_bits = encoding.mod_indicator();

        data_segment.append(&mut mod_indicator_bits);
        data_segment.append(&mut char_count_to_bits);
        data_segment.append(&mut bits);

        // Compute total size without ec bits
        let (block_1_size, block_1_count, block_2_size, block_2_count) =
            DATA_BYTES_PER_BLOCK[version - 1][ec_level.ordinal() as usize];
        let total_data_bits = (block_1_size * block_1_count + block_2_size * block_2_count) * 8;

        // Add terminator bits (at most 4 0s)
        if data_segment.len() < total_data_bits {
            let empty_bits = total_data_bits - data_segment.len();
            if empty_bits >= 4 {
                data_segment.append(&mut vec![Bit::Zero(false); 4]);
            } else {
                data_segment.append(&mut vec![Bit::Zero(false); empty_bits]);
            }
        }

        // Add padding bits
        while data_segment.len() % 8 != 0 {
            data_segment.push(Bit::Zero(false));
        }

        // Add padding bytes
        if data_segment.len() < total_data_bits {
            let mut byte_1 = to_bits_array(&[236]);
            let mut byte_2 = to_bits_array(&[17]);

            while data_segment.len() < total_data_bits {
                data_segment.extend_from_slice(&byte_1);
                std::mem::swap(&mut byte_1, &mut byte_2);
            }
        }

        let cw_per_block = EC_BYTES_PER_BLOCK[version - 1][ec_level.ordinal() as usize];

        let (data_codewords, ec_codewords) = codewords(
            &Bit::bytes(&data_segment),
            version as u8,
            &ec_level,
            cw_per_block,
        );

        let mut data_bits = Bit::bits(&data_codewords, data_codewords.len() * 8);
        let error_correction = Bit::bits(&ec_codewords, ec_codewords.len() * 8);

        data_bits.extend(error_correction);

        debug_vec!(&data_bits);

        Preprocessor {
            qrcode_bits: data_bits,
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
