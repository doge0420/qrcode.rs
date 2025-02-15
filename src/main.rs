use crate::ec::*;
use crate::encoding::Encoding;
use crate::preprocessor::Preprocessor;

mod bit;
mod ec;
mod encoding;
mod format;
mod preprocessor;
mod qrcode;
mod types;

fn main() {
    let data = "https://example.com";
    let preprocessor = Preprocessor::new(data, Encoding::Byte, EcLevel::L);
    let mask_pattern = 0;
    let qrcode = preprocessor.generate_qrcode(mask_pattern);
    println!("{}", qrcode);
}
