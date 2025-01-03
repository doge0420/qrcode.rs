use crate::types::{EcLevel, QrCode};

mod types;

fn main() {
    let mut qrcode = QrCode::new(8, EcLevel::L).unwrap();
    qrcode.finder_patterns();
    qrcode.separators_patterns();
    qrcode.alignment_patterns();
    println!("{}", qrcode);
}
