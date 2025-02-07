use crate::bit::Bit;
use crate::encoding::{to_bits, with_mod_indicator, Encoding};
use crate::qrcode::{EcLevel, QrCode};

mod bit;
mod encoding;
mod qrcode;
mod types;

fn main() {
    let mut qrcode = QrCode::new(2, EcLevel::L, 4, Encoding::Alphanumeric).unwrap();
    qrcode.all_functional_patterns();

    let mut data = to_bits("hello world".to_string());

    with_mod_indicator(&mut data, Encoding::Alphanumeric);

    for bit in data.iter() {
        match bit {
            Bit::One(_) => {
                print!("1")
            }
            Bit::Zero(_) => {
                print!("0")
            }
        }
    }

    println!();

    qrcode.fill(data);

    println!("{}", qrcode);
}
