use crate::bit::Bit;
use crate::ec::*;
use crate::encoding::*;
use crate::mask::MaskPattern;
use crate::qrcode::QrCode;

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

        let sizes = match ec_level {
            EcLevel::H => SIZE_EC_H,
            EcLevel::Q => SIZE_EC_Q,
            EcLevel::M => SIZE_EC_M,
            EcLevel::L => SIZE_EC_L,
        };

        let size = sizes[version - 1] as usize;

        let mut char_count = Bit::from(
            data.len() as u32,
            Self::char_count(version as u8, encoding),
            false,
            true,
        );

        if bits.len() < size {
            bits.append(&mut vec![Bit::Zero(false); size - bits.len()]);
        }

        let mut qrcode_bits = encoding.mod_indicator();
        let bytes = Bit::bytes(&bits);
        let mut error_correction = Bit::bits(&error_correction(&bytes, version as u8, &ec_level));

        qrcode_bits.append(&mut char_count);
        qrcode_bits.append(&mut Bit::bits(&interleave(groups(
            &bytes,
            version as u8,
            &ec_level,
        ))));

        qrcode_bits.append(&mut error_correction);

        for bit in &qrcode_bits {
            if !bit.is_functional() {
                if bit.value() {
                    print!("1");
                } else {
                    print!("0");
                }
            }
        }
        println!();

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

/// 0: version 1 - 9, 1: version 10 - 26, 2: version 27 - 40
static NUMERIC_CHAR_COUNT: [u8; 3] = [10, 12, 14];

/// 0: version 1 - 9, 1: version 10 - 26, 2: version 27 - 40
static ALPHANUMERIC_CHAR_COUNT: [u8; 3] = [9, 11, 13];

/// 0: version 1 - 9, 1: version 10 - 26, 2: version 27 - 40
static BYTE_CHAR_COUNT: [u8; 3] = [8, 16, 16];

/// 0: version 1 - 9, 1: version 10 - 26, 2: version 27 - 40
static KANJI_CHAR_COUNT: [u8; 3] = [8, 10, 12];

static SIZE_EC_L: [u32; 40] = [
    19, 34, 55, 80, 108, 136, 156, 194, 232, 274, 324, 370, 428, 461, 523, 589, 647, 721, 795, 861,
    932, 1006, 1094, 1174, 1276, 1370, 1468, 1531, 1631, 1735, 1843, 1955, 2071, 2191, 2306, 2434,
    2566, 2702, 2812, 2956,
];

static SIZE_EC_M: [u32; 40] = [
    16, 28, 44, 64, 86, 108, 124, 154, 182, 216, 254, 290, 334, 365, 415, 453, 507, 563, 627, 669,
    714, 782, 860, 914, 1000, 1062, 1128, 1193, 1267, 1373, 1455, 1541, 1631, 1725, 1812, 1914,
    1992, 2102, 2216, 2334,
];

static SIZE_EC_Q: [u32; 40] = [
    13, 22, 34, 48, 62, 76, 88, 110, 132, 154, 180, 206, 244, 261, 295, 325, 367, 397, 445, 485,
    512, 568, 614, 664, 718, 754, 808, 871, 911, 985, 1033, 1115, 1171, 1231, 1286, 1354, 1426,
    1502, 1582, 1666,
];

static SIZE_EC_H: [u32; 40] = [
    9, 16, 26, 36, 46, 60, 66, 86, 100, 122, 140, 158, 180, 197, 223, 253, 283, 313, 341, 385, 406,
    442, 464, 514, 538, 596, 628, 661, 701, 745, 793, 845, 901, 961, 986, 1054, 1096, 1142, 1222,
    1276,
];

static NUMERIC_SIZE: [u32; 160] = [
    41, 34, 27, 17, 77, 63, 48, 34, 127, 101, 77, 58, 187, 149, 111, 82, 255, 202, 144, 106, 322,
    255, 178, 139, 370, 293, 207, 154, 461, 365, 259, 202, 552, 432, 312, 235, 652, 513, 364, 288,
    772, 604, 427, 331, 883, 691, 489, 374, 1022, 796, 580, 427, 1101, 871, 621, 468, 1250, 991,
    703, 530, 1408, 1082, 775, 602, 1548, 1212, 876, 674, 1725, 1346, 948, 746, 1903, 1500, 1063,
    813, 2061, 1600, 1159, 919, 2232, 1708, 1224, 969, 2409, 1872, 1358, 1056, 2620, 2059, 1468,
    1108, 2812, 2188, 1588, 1228, 3057, 2395, 1718, 1286, 3283, 2544, 1804, 1425, 3517, 2701, 1933,
    1501, 3669, 2857, 2085, 1581, 3909, 3035, 2181, 1677, 4158, 3289, 2358, 1782, 4417, 3486, 2473,
    1897, 4686, 3693, 2670, 2022, 4965, 3909, 2805, 2157, 5253, 4134, 2949, 2301, 5529, 4343, 3081,
    2361, 5836, 4588, 3244, 2524, 6153, 4775, 3417, 2625, 6479, 5039, 3599, 2735, 6743, 5313, 3791,
    2927, 7089, 5596, 3993, 3057,
];

static ALPHANUMERIC_SIZE: [u32; 160] = [
    25, 20, 16, 10, 47, 38, 29, 20, 77, 61, 47, 35, 114, 90, 67, 50, 154, 122, 87, 64, 195, 154,
    108, 84, 224, 178, 125, 93, 279, 221, 157, 122, 335, 262, 189, 143, 395, 311, 221, 174, 468,
    366, 259, 200, 535, 419, 296, 227, 619, 483, 352, 259, 667, 528, 376, 283, 758, 600, 426, 321,
    854, 656, 470, 365, 938, 734, 531, 408, 1046, 816, 574, 452, 1153, 909, 644, 493, 1249, 970,
    702, 557, 1352, 1035, 742, 587, 1460, 1134, 823, 640, 1588, 1248, 890, 672, 1704, 1326, 963,
    744, 1853, 1451, 1041, 779, 1990, 1542, 1094, 864, 2132, 1637, 1172, 910, 2223, 1732, 1263,
    958, 2369, 1839, 1322, 1016, 2520, 1994, 1429, 1080, 2677, 2113, 1499, 1150, 2840, 2238, 1618,
    1226, 3009, 2369, 1700, 1307, 3183, 2506, 1787, 1394, 3351, 2632, 1867, 1431, 3537, 2780, 1966,
    1530, 3729, 2894, 2071, 1591, 3927, 3054, 2181, 1658, 4087, 3220, 2298, 1774, 4296, 3391, 2420,
    1852,
];

static BYTE_SIZE: [u32; 160] = [
    17, 14, 11, 7, 32, 26, 20, 14, 53, 42, 32, 24, 78, 62, 46, 34, 106, 84, 60, 44, 134, 106, 74,
    58, 154, 122, 86, 64, 192, 152, 108, 84, 230, 180, 130, 98, 271, 213, 151, 119, 321, 251, 177,
    137, 367, 287, 203, 155, 425, 331, 241, 177, 458, 362, 258, 194, 520, 412, 292, 220, 586, 450,
    322, 250, 644, 504, 364, 280, 718, 560, 394, 310, 792, 624, 442, 338, 858, 666, 482, 382, 929,
    711, 509, 403, 1003, 779, 565, 439, 1091, 857, 611, 461, 1171, 911, 661, 511, 1273, 997, 715,
    535, 1367, 1059, 751, 593, 1465, 1125, 805, 625, 1528, 1190, 868, 658, 1628, 1264, 908, 698,
    1732, 1370, 982, 742, 1840, 1452, 1030, 790, 1952, 1538, 1112, 842, 2068, 1628, 1168, 898,
    2188, 1722, 1228, 958, 2303, 1809, 1283, 983, 2431, 1911, 1351, 1051, 2563, 1989, 1423, 1093,
    2699, 2099, 1499, 1139, 2809, 2213, 1579, 1219, 2953, 2331, 1663, 1273,
];

static KANJI_SIZE: [u32; 160] = [
    10, 8, 7, 4, 20, 16, 12, 8, 32, 26, 20, 15, 48, 38, 28, 21, 65, 52, 37, 27, 82, 65, 45, 36, 95,
    75, 53, 39, 118, 93, 66, 52, 141, 111, 80, 60, 167, 131, 93, 74, 198, 155, 109, 85, 226, 177,
    125, 96, 262, 204, 149, 109, 282, 223, 159, 120, 320, 254, 180, 136, 361, 277, 198, 154, 397,
    310, 224, 173, 442, 345, 243, 191, 488, 384, 272, 208, 528, 410, 297, 235, 572, 438, 314, 248,
    618, 480, 348, 270, 672, 528, 376, 284, 721, 561, 407, 315, 784, 614, 440, 330, 842, 652, 462,
    365, 902, 692, 496, 385, 940, 732, 534, 405, 1002, 778, 559, 430, 1066, 843, 604, 457, 1132,
    894, 634, 486, 1201, 947, 684, 518, 1273, 1002, 719, 553, 1347, 1060, 756, 590, 1417, 1113,
    790, 605, 1496, 1176, 832, 647, 1577, 1224, 876, 673, 1661, 1292, 923, 701, 1729, 1362, 972,
    750, 1817, 1435, 1024, 784,
];
