use crate::bit::Bit;
use crate::ec::EcLevel;
use crate::encoding::Encoding;
use crate::mask::MaskPattern;
use std::fmt;
use std::fmt::Formatter;

pub struct QrCode {
    pub data: Vec<Bit>,
    version: u8,
    ec_level: EcLevel,
    mask_pattern: MaskPattern,
    encoding: Encoding,
}

impl QrCode {
    fn get(&self, x: u32, y: u32) -> Option<Bit> {
        if let Some(index) = self.coords_to_index_from_instance(x, y) {
            self.data.get(index as usize).copied()
        } else {
            None
        }
    }

    fn put(&mut self, x: u32, y: u32, data: Bit) {
        if let Some(index) = self.coords_to_index_from_instance(x, y) {
            self.data[index as usize] = data;
        }
    }

    fn coords_to_index(x: u32, y: u32, size: u32) -> Option<u32> {
        if !(x < size && y < size) {
            None
        } else {
            Some(x + size * y)
        }
    }

    fn coords_to_index_from_version(x: u32, y: u32, version: u8) -> Option<u32> {
        Self::coords_to_index(x, y, Self::size_from_version(version))
    }

    fn coords_to_index_from_instance(&self, x: u32, y: u32) -> Option<u32> {
        Self::coords_to_index(x, y, self.size())
    }

    pub fn new(
        version: u8,
        ec_level: EcLevel,
        mask_pattern: MaskPattern,
        encoding: Encoding,
    ) -> Result<QrCode, String> {
        if version > 40 || version == 0 {
            Err("Invalid version.".to_string())
        } else {
            let size = Self::size_from_version(version);
            let data = vec![Bit::Zero(false); (size * size) as usize];
            Ok(QrCode {
                data,
                version,
                ec_level,
                mask_pattern,
                encoding,
            })
        }
    }

    fn size_from_version(version: u8) -> u32 {
        17 + 4 * version as u32
    }

    fn size(&self) -> u32 {
        Self::size_from_version(self.version)
    }

    fn finder_patterns(&mut self) {
        const FINDER_PATTERN: [Bit; 49] = [
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
        ];

        const PATTERN_LENGTH: u32 = 7;

        let size = self.size();
        let corners = [(0, 0), (size - 7, 0), (0, size - 7)];

        for corner in corners {
            let (x, y) = corner;
            for dx in 0..PATTERN_LENGTH {
                for dy in 0..PATTERN_LENGTH {
                    self.put(
                        x + dx,
                        y + dy,
                        FINDER_PATTERN[(dx + PATTERN_LENGTH * dy) as usize],
                    )
                }
            }
        }
    }

    fn separators_patterns(&mut self) {
        let size = self.size();
        let top = [(7, 0), (size - 8, 0), (7, size - 8)];
        let right = [(0, 7), (size - 7, 7), (0, size - 8)];

        for (x, y) in top {
            for dy in 0..8 {
                self.put(x, y + dy, Bit::Zero(true))
            }
        }

        for (x, y) in right {
            for dx in 0..7 {
                self.put(x + dx, y, Bit::Zero(true))
            }
        }
    }

    fn combination(array: &[u8]) -> Vec<(u8, u8)> {
        let mut res: Vec<(u8, u8)> = vec![];

        for elem1 in array {
            for elem2 in array {
                res.push((elem1.clone(), elem2.clone()));
            }
        }

        res
    }

    fn draw_alignment_pattern(&mut self, x: u32, y: u32) {
        let cx = x - 2;
        let cy = y - 2;

        const PATTERN_LENGTH: u32 = 5;

        const ALIGNMENT_PATTERN: [Bit; 25] = [
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::Zero(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
            Bit::One(true),
        ];

        if !self.get(x, y).unwrap().is_functional() {
            for dx in 0..PATTERN_LENGTH {
                for dy in 0..PATTERN_LENGTH {
                    self.put(
                        cx + dx,
                        cy + dy,
                        ALIGNMENT_PATTERN[(dx + PATTERN_LENGTH * dy) as usize],
                    )
                }
            }
        }
    }

    fn alignment_patterns(&mut self) {
        const COORDS: [&[u8]; 39] = [
            &[6, 18],
            &[6, 22],
            &[6, 26],
            &[6, 30],
            &[6, 34],
            &[6, 22, 38],
            &[6, 24, 42],
            &[6, 26, 46],
            &[6, 28, 50],
            &[6, 30, 54],
            &[6, 32, 58],
            &[6, 34, 62],
            &[6, 26, 46, 66],
            &[6, 26, 48, 70],
            &[6, 26, 50, 74],
            &[6, 30, 54, 78],
            &[6, 30, 56, 82],
            &[6, 30, 58, 86],
            &[6, 34, 62, 90],
            &[6, 28, 50, 72, 94],
            &[6, 26, 50, 74, 98],
            &[6, 30, 54, 78, 102],
            &[6, 28, 54, 80, 106],
            &[6, 32, 58, 84, 110],
            &[6, 30, 58, 86, 114],
            &[6, 34, 62, 90, 118],
            &[6, 26, 50, 74, 98, 122],
            &[6, 30, 54, 78, 102, 126],
            &[6, 26, 52, 78, 104, 130],
            &[6, 30, 56, 82, 108, 134],
            &[6, 34, 60, 86, 112, 138],
            &[6, 30, 58, 86, 114, 142],
            &[6, 34, 62, 90, 118, 146],
            &[6, 30, 54, 78, 102, 126, 150],
            &[6, 24, 50, 76, 102, 128, 154],
            &[6, 28, 54, 80, 106, 132, 158],
            &[6, 32, 58, 84, 110, 136, 162],
            &[6, 26, 54, 82, 110, 138, 166],
            &[6, 30, 58, 86, 114, 142, 170],
        ];

        if self.version != 1 {
            let combinations = Self::combination(COORDS[(self.version - 2) as usize]);
            for (x, y) in combinations {
                self.draw_alignment_pattern(x as u32, y as u32);
            }
        }
    }

    fn timing_patterns(&mut self) {
        let length = self.size() - 16;

        let mut bit;
        for dx in 0..length {
            let x = dx + 8;

            if self.get(x, 6).unwrap().is_functional() {
                continue;
            }

            if x % 2 == 0 {
                bit = Bit::One(true);
            } else {
                bit = Bit::Zero(true);
            }

            self.put(x, 6, bit);
        }

        for dy in 0..length {
            let y = dy + 8;

            if self.get(6, y).unwrap().is_functional() {
                continue;
            }

            if y % 2 == 0 {
                bit = Bit::One(true);
            } else {
                bit = Bit::Zero(true);
            }

            self.put(6, y, bit);
        }
    }

    fn dark_module(&mut self) {
        self.put(8, (4 * self.version + 9) as u32, Bit::One(true))
    }

    fn format_information(&mut self) {
        const FORMAT_BITS: [u32; 32] = [
            0x77C4, 0x72F3, 0x7DAA, 0x789D, 0x662F, 0x6318, 0x6C41, 0x6976, 0x5412, 0x5125, 0x5E7C,
            0x5B4B, 0x45F9, 0x40CE, 0x4F97, 0x4AA0, 0x355F, 0x3068, 0x3F31, 0x3A06, 0x24B4, 0x2183,
            0x2EDA, 0x2BED, 0x1689, 0x13BE, 0x1CE7, 0x19D0, 0x762, 0x255, 0xD0C, 0x83B,
        ];

        let mut index = self.mask_pattern.ordinal() as u32;
        match self.ec_level {
            EcLevel::L => index += 0,
            EcLevel::M => index += 8 * 1,
            EcLevel::Q => index += 8 * 2,
            EcLevel::H => index += 8 * 3,
        }

        let info_bit = FORMAT_BITS[index as usize];
        let bits = Bit::from(info_bit, 15, true, true);

        let mut i = 0;
        for x in 0..9 {
            if self.get(x, 8).unwrap().is_functional() {
                continue;
            }
            self.put(x, 8, bits[i as usize]);
            i += 1;
        }

        let mut i = 8;
        for y in (0..8).rev() {
            if self.get(8, y).unwrap().is_functional() {
                continue;
            }
            self.put(8, y, bits[i as usize]);
            i += 1;
        }

        let mut i = 0;
        for y in ((self.size() - 7)..self.size()).rev() {
            self.put(8, y, bits[i as usize]);
            i += 1;
        }

        let mut i = 7;
        for x in (self.size() - 8)..self.size() {
            self.put(x, 8, bits[i as usize]);
            i += 1;
        }
    }

    fn version_information(&mut self) {
        assert!(
            self.version >= 7,
            "Version information is not available for versions below 7."
        );

        const VERSION_BITS: [u32; 34] = [
            0x07c94, 0x085bc, 0x09a99, 0x0a4d3, 0x0bbf6, 0x0c762, 0x0d847, 0x0e60d, 0x0f928,
            0x10b78, 0x1145d, 0x12a17, 0x13532, 0x149a6, 0x15683, 0x168c9, 0x177ec, 0x18ec4,
            0x191e1, 0x1afab, 0x1b08e, 0x1cc1a, 0x1d33f, 0x1ed75, 0x1f250, 0x209d5, 0x216f0,
            0x228ba, 0x2379f, 0x24b0b, 0x2542e, 0x26a64, 0x27541, 0x28c69,
        ];

        let version_bits = VERSION_BITS[(self.version - 7) as usize];
        let bits = Bit::from(version_bits, 18, true, true);

        // bottom left
        let mut x = 0;
        let mut y = self.size() - 11;
        for i in 0..18 {
            if i % 3 == 0 && i != 0 {
                x += 1;
                y = self.size() - 11;
            }
            self.put(x, y, bits[i as usize]);
            y += 1;
        }

        // top right
        let mut x = self.size() - 11;
        let mut y = 0;
        for i in 0..18 {
            if i % 3 == 0 && i != 0 {
                y += 1;
                x = self.size() - 11;
            }
            self.put(x, y, bits[i as usize]);
            x += 1;
        }
    }

    pub fn apply_mask(&mut self) {
        let mask_fn = self.mask_pattern.get_mask();

        for x in 0..self.size() {
            for y in 0..self.size() {
                let bit = self.get(x, y).unwrap();
                if !bit.is_functional() {
                    if mask_fn(x, y) {
                        self.put(x, y, bit.invert());
                    }
                }
            }
        }
    }

    pub fn all_functional_patterns(&mut self) {
        self.finder_patterns();
        self.separators_patterns();
        self.alignment_patterns();
        self.timing_patterns();
        self.dark_module();
        self.format_information();
        if self.version >= 7 {
            self.version_information();
        }
    }

    // pub fn fill(&mut self, bits: &Vec<Bit>) {
    //     let mut x = self.size() as usize - 1;
    //     let mut y = self.size() as usize - 1;
    //     let mut up = true;
    //
    //     let mut i = 0;
    //
    //     for bit in bits {
    //         // println!("{}", &self);
    //         loop {
    //             match up {
    //                 true => {
    //                     // if we are at the top, go down and change column
    //                     if y <= 0 {
    //                         up = false;
    //                         x -= 1;
    //                         // skip the timing pattern
    //                         if x == 7 {
    //                             x -= 1;
    //                         }
    //                         y = 0;
    //                     } else {
    //                         if i % 2 == 0 {
    //                             x -= 1;
    //                         } else {
    //                             x += 1;
    //                             y -= 1;
    //                         }
    //                         i += 1;
    //                     }
    //                 }
    //                 false => {
    //                     // if we are at the bottom, go up and change column
    //                     if y >= self.size() as usize - 1 {
    //                         up = true;
    //                         x -= 1;
    //                         // skip the timing pattern
    //                         if x == 7 {
    //                             x -= 1;
    //                         }
    //                         y = self.size() as usize - 1;
    //                     } else {
    //                         if i % 2 == 0 && x != 0 {
    //                             x -= 1;
    //                         } else {
    //                             x += 1;
    //                             y += 1;
    //                         }
    //                         i += 1;
    //                     }
    //                 }
    //             }
    //
    //             if !self.get(x as u32, y as u32).unwrap().is_functional() {
    //                 break;
    //             }
    //         }
    //
    //         self.put(x as u32, y as u32, *bit);
    //     }
    // }

    pub fn fill(&mut self, bits: &Vec<Bit>) {
        let n = self.size() as usize;
        let mut bit_iter = bits.iter();
        let mut col = n - 1;
        // The filling direction alternates: true = upward (bottom-to-top), false = downward.
        let mut upward = true;

        while col > 0 {
            // Skip the vertical timing pattern column if needed.
            if col == 7 {
                col -= 1;
            }

            if upward {
                // Process rows from bottom (n-1) up to top (0).
                for row in (0..n).rev() {
                    // In a pair of columns, we fill the module in col-0 and col-1
                    for offset in 0..2 {
                        let x = col - offset;
                        // Only fill if the cell is not reserved for a function pattern.
                        if !self.get(x as u32, row as u32).unwrap().is_functional() {
                            if let Some(bit) = bit_iter.next() {
                                self.put(x as u32, row as u32, *bit);
                            } else {
                                return;
                            }
                        }
                    }
                }
            } else {
                // Process rows from top (0) down to bottom (n-1).
                for row in 0..n {
                    for offset in 0..2 {
                        let x = col - offset;
                        if !self.get(x as u32, row as u32).unwrap().is_functional() {
                            if let Some(bit) = bit_iter.next() {
                                self.put(x as u32, row as u32, *bit);
                            } else {
                                return;
                            }
                        }
                    }
                }
            }
            // After finishing a pair of columns, flip the direction and move left two columns.
            upward = !upward;
            col -= 2;
        }
    }
}

impl fmt::Display for QrCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut qrcode = String::new();

        let lines: Vec<_> = self.data.chunks(self.size() as usize).collect();

        // add quiet zone
        for _ in 0..4 {
            for _ in 0..self.size() + 8 {
                qrcode.push_str("██");
            }
            qrcode.push('\n');
        }

        for i in 0..self.size() {
            for _ in 0..4 {
                qrcode.push_str("██");
            }
            for module in lines[i as usize] {
                if module.value() {
                    qrcode.push_str("  ");
                } else {
                    qrcode.push_str("██");
                }
            }
            for _ in 0..4 {
                qrcode.push_str("██");
            }
            qrcode.push('\n');
        }

        for _ in 0..4 {
            for _ in 0..self.size() + 8 {
                qrcode.push_str("██");
            }
            qrcode.push('\n');
        }

        let mut version = String::from('\n');
        for _ in 0..self.size() {
            version.push(' ');
        }
        version.push_str("Version: ");
        version.push_str(self.version.to_string().as_str());
        version.push('\n');

        write!(f, "{}{}", qrcode, version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qrcode::Bit::{One, Zero};

    #[test]
    fn get_returns_correct_bit() {
        let qr = QrCode::new(
            1,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        assert!(matches!(qr.get(0, 0), Some(Zero(_))));
    }

    #[test]
    fn get_returns_none_for_out_of_bounds() {
        let qr = QrCode::new(
            1,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        assert_eq!(qr.get(100, 100), None);
    }

    #[test]
    fn new_returns_error_for_invalid_version() {
        let result = QrCode::new(
            41,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        );
        assert!(result.is_err());
    }

    #[test]
    fn new_creates_qrcode_with_correct_size() {
        let qr = QrCode::new(
            1,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        assert_eq!(qr.size(), 21);
    }

    #[test]
    fn size_from_version_calculates_correct_size() {
        assert_eq!(QrCode::size_from_version(1), 21);
        assert_eq!(QrCode::size_from_version(40), 177);
    }

    #[test]
    fn new_creates_qrcode_with_valid_version() {
        let qr = QrCode::new(
            10,
            EcLevel::M,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        assert_eq!(qr.version, 10);
        assert_eq!(qr.size(), 57);
    }

    #[test]
    fn new_creates_qrcode_with_correct_ec_level() {
        let qr = QrCode::new(
            5,
            EcLevel::Q,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        match qr.ec_level {
            EcLevel::Q => assert!(true),
            _ => assert!(false, "Expected EcLevel::Q"),
        }
    }

    #[test]
    fn new_creates_qrcode_with_correct_data_size() {
        let qr = QrCode::new(
            2,
            EcLevel::H,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        assert_eq!(qr.data.len(), 625);
    }

    #[test]
    fn new_returns_error_for_zero_version() {
        let result = QrCode::new(
            0,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        );
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid version.".to_string()));
    }

    #[test]
    fn new_returns_error_for_negative_version() {
        let result = QrCode::new(
            -1i8 as u8,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        );
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid version.".to_string()));
    }

    #[test]
    fn size_from_version_calculates_size_for_min_version() {
        assert_eq!(QrCode::size_from_version(1), 21);
    }

    #[test]
    fn size_from_version_calculates_size_for_max_version() {
        assert_eq!(QrCode::size_from_version(40), 177);
    }

    #[test]
    fn size_from_version_calculates_size_for_intermediate_version() {
        assert_eq!(QrCode::size_from_version(20), 97);
    }

    #[test]
    fn size_from_version_calculates_size_for_large_version() {
        assert_eq!(QrCode::size_from_version(100), 417);
    }

    #[test]
    fn finder_patterns_creates_correct_patterns() {
        let mut qr = QrCode::new(
            1,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        qr.finder_patterns();
        let expected_pattern = [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (0, 1),
            (6, 1),
            (0, 2),
            (6, 2),
            (0, 3),
            (6, 3),
            (0, 4),
            (6, 4),
            (0, 5),
            (6, 5),
            (0, 6),
            (1, 6),
            (2, 6),
            (3, 6),
            (4, 6),
            (5, 6),
            (6, 6),
        ];
        for &(x, y) in &expected_pattern {
            assert_eq!(qr.get(x, y), Some(One(true)));
        }
    }

    #[test]
    fn finder_patterns_handles_minimum_size() {
        let mut qr = QrCode::new(
            1,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        qr.finder_patterns();
        assert!(matches!(qr.get(0, 0), Some(One(_))));
        assert!(matches!(qr.get(20, 20), Some(Zero(_))));
    }

    #[test]
    fn finder_patterns_handles_maximum_size() {
        let mut qr = QrCode::new(
            40,
            EcLevel::L,
            MaskPattern::Checkerboard,
            Encoding::Alphanumeric,
        )
        .unwrap();
        qr.finder_patterns();
        assert!(matches!(qr.get(0, 0), Some(One(_))));
        assert!(matches!(qr.get(176, 176), Some(Zero(_))));
    }
}
