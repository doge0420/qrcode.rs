use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
enum Bit {
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
    fn is_functional(&self) -> bool {
        match self {
            Bit::One(val) => *val,
            Bit::Zero(val) => *val,
        }
    }

    fn value(&self) -> bool {
        match self {
            Bit::One(_) => true,
            Bit::Zero(_) => false,
        }
    }
}

pub enum EcLevel {
    H,
    Q,
    M,
    L,
}

pub struct QrCode {
    data: Vec<Bit>,
    version: u8,
    ec_level: EcLevel,
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

    pub fn new(version: u8, ec_level: EcLevel) -> Result<QrCode, String> {
        if version > 40 || version == 0 {
            Err("Invalid version.".to_string())
        } else {
            let size = Self::size_from_version(version);
            let data = vec![Bit::Zero(false); (size * size) as usize];
            Ok(QrCode {
                data,
                version,
                ec_level,
            })
        }
    }

    fn size_from_version(version: u8) -> u32 {
        17 + 4 * version as u32
    }

    fn size(&self) -> u32 {
        Self::size_from_version(self.version)
    }

    pub fn finder_patterns(&mut self) {
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

    pub fn separators_patterns(&mut self) {
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

    pub fn alignment_patterns(&mut self) {
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

        if self.version == 1 {
            self.draw_alignment_pattern(18, 18);
        } else {
            let combinations = Self::combination(COORDS[(self.version - 2) as usize]);
            for (x, y) in combinations {
                self.draw_alignment_pattern(x as u32, y as u32);
            }
        }
    }
}

impl fmt::Display for QrCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut qrcode = String::new();

        for (i, module) in self.data.iter().enumerate() {
            if module.value() {
                qrcode.push_str("  ");
            } else {
                qrcode.push_str("██");
            }

            if (i + 1) % self.size() as usize == 0 {
                qrcode.push('\n')
            }
        }

        let mut version = String::from('\n');
        for _ in 0..(self.size() - 5) {
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
    use crate::types::Bit::{One, Zero};

    #[test]
    fn get_returns_correct_bit() {
        let qr = QrCode::new(1, EcLevel::L).unwrap();
        assert!(matches!(qr.get(0, 0), Some(Zero(_))));
    }

    #[test]
    fn get_returns_none_for_out_of_bounds() {
        let qr = QrCode::new(1, EcLevel::L).unwrap();
        assert_eq!(qr.get(100, 100), None);
    }

    #[test]
    fn new_returns_error_for_invalid_version() {
        let result = QrCode::new(41, EcLevel::L);
        assert!(result.is_err());
    }

    #[test]
    fn new_creates_qrcode_with_correct_size() {
        let qr = QrCode::new(1, EcLevel::L).unwrap();
        assert_eq!(qr.size(), 21);
    }

    #[test]
    fn size_from_version_calculates_correct_size() {
        assert_eq!(QrCode::size_from_version(1), 21);
        assert_eq!(QrCode::size_from_version(40), 177);
    }

    #[test]
    fn new_creates_qrcode_with_valid_version() {
        let qr = QrCode::new(10, EcLevel::M).unwrap();
        assert_eq!(qr.version, 10);
        assert_eq!(qr.size(), 57);
    }

    #[test]
    fn new_creates_qrcode_with_correct_ec_level() {
        let qr = QrCode::new(5, EcLevel::Q).unwrap();
        match qr.ec_level {
            EcLevel::Q => assert!(true),
            _ => assert!(false, "Expected EcLevel::Q"),
        }
    }

    #[test]
    fn new_creates_qrcode_with_correct_data_size() {
        let qr = QrCode::new(2, EcLevel::H).unwrap();
        assert_eq!(qr.data.len(), 625);
    }

    #[test]
    fn new_returns_error_for_zero_version() {
        let result = QrCode::new(0, EcLevel::L);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Invalid version.".to_string()));
    }

    #[test]
    fn new_returns_error_for_negative_version() {
        let result = QrCode::new(-1i8 as u8, EcLevel::L);
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
        let mut qr = QrCode::new(1, EcLevel::L).unwrap();
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
        let mut qr = QrCode::new(1, EcLevel::L).unwrap();
        qr.finder_patterns();
        assert!(matches!(qr.get(0, 0), Some(One(_))));
        assert!(matches!(qr.get(20, 20), Some(Zero(_))));
    }

    #[test]
    fn finder_patterns_handles_maximum_size() {
        let mut qr = QrCode::new(40, EcLevel::L).unwrap();
        qr.finder_patterns();
        assert!(matches!(qr.get(0, 0), Some(One(_))));
        assert!(matches!(qr.get(176, 176), Some(Zero(_))));
    }
}
