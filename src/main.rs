use crate::ec::*;
use crate::encoding::Encoding;
use crate::qrcode::QrCode;

mod bit;
mod ec;
mod encoding;
mod preprocessor;
mod qrcode;
mod types;

fn main() {
    let mut qrcode = QrCode::new(2, EcLevel::L, 4, Encoding::Alphanumeric).unwrap();
    qrcode.all_functional_patterns();
    //
    // // let mut data = to_bits("hello world".to_string());
    // //
    // // with_mod_indicator(&mut data, Encoding::Alphanumeric);
    //
    // for bit in data.iter() {
    //     match bit {
    //         Bit::One(_) => {
    //             print!("1")
    //         }
    //         Bit::Zero(_) => {
    //             print!("0")
    //         }
    //     }
    // }
    //
    // println!();
    //
    // qrcode.fill(data);
    //
    // println!("{}", qrcode);
}
