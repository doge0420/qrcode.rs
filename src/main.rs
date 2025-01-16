use crate::types::{EcLevel, QrCode};

mod types;

fn main() {
    let mut qrcode = QrCode::new(3, EcLevel::L, 1).unwrap();
    qrcode.finder_patterns();
    qrcode.separators_patterns();
    qrcode.alignment_patterns();
    qrcode.timing_patterns();
    qrcode.dark_module();
    println!("{}", qrcode);
}
