use crate::types::{Bit, EcLevel, QrCode};

mod types;

fn main() {
    let mut qrcode = QrCode::new(3, EcLevel::L, 4).unwrap();
    qrcode.finder_patterns();
    qrcode.separators_patterns();
    qrcode.alignment_patterns();
    qrcode.timing_patterns();
    qrcode.dark_module();
    qrcode.format_information();
    // qrcode.version_information();

    let data = 0xAA;
    let bits = Bit::from(data, 8, false, false);
    qrcode.fill(bits);

    println!("{}", qrcode);
}
