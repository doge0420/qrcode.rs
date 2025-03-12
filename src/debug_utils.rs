#[macro_export]
macro_rules! debug_vec {
    ($bits:expr) => {
        #[cfg(debug_assertions)]
        {
            let mut res = String::new();
            $bits.iter().for_each(|bit| {
                if bit.value() {
                    res.push('1');
                } else {
                    res.push('0');
                }
            });
            res.push('\n');
            println!("{}", res);
        }
    };
}
