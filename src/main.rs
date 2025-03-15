use crate::ec::*;
use crate::encoding::Encoding;
use crate::mask::MaskPattern;
use crate::preprocessor::Preprocessor;

mod bit;
mod debug_utils;
mod ec;
mod encoding;
mod format;
mod mask;
mod preprocessor;
mod qrcode;
mod tables;

fn main() {
    let data = "696969420";
    let preprocessor =
        Preprocessor::new(data, Encoding::Numeric, EcLevel::Q, MaskPattern::Diagonal);
    let qrcode = preprocessor.generate_qrcode();

    println!("{}", qrcode);
}
