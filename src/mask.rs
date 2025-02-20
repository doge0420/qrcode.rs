#[derive(Clone, Copy)]
pub enum MaskPattern {
    Checkerboard,
    Horizontal,
    Vertical,
    Diagonal,
    LargeCheckerboard,
    Fields,
    Diamonds,
    Meadow,
}

impl MaskPattern {
    pub fn get_mask(&self) -> fn(u32, u32) -> bool {
        match self {
            MaskPattern::Checkerboard => |x, y| (x + y) % 2 == 0,
            MaskPattern::Horizontal => |_, y| y % 2 == 0,
            MaskPattern::Vertical => |x, _| x % 3 == 0,
            MaskPattern::Diagonal => |x, y| (x + y) % 3 == 0,
            MaskPattern::LargeCheckerboard => |x, y| (x / 2 + y / 3) % 2 == 0,
            MaskPattern::Fields => |x, y| (x * y) % 2 + (x * y) % 3 == 0,
            MaskPattern::Diamonds => |x, y| ((x * y) % 2 + (x * y) % 3) % 2 == 0,
            MaskPattern::Meadow => |x, y| ((x + y) % 2 + (x * y) % 3) % 2 == 0,
        }
    }

    pub fn ordinal(self) -> u8 {
        match self {
            MaskPattern::Checkerboard => 0,
            MaskPattern::Horizontal => 1,
            MaskPattern::Vertical => 2,
            MaskPattern::Diagonal => 3,
            MaskPattern::LargeCheckerboard => 4,
            MaskPattern::Fields => 5,
            MaskPattern::Diamonds => 6,
            MaskPattern::Meadow => 7,
        }
    }
}
