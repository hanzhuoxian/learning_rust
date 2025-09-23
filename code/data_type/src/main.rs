fn main() {
    let xi8 = 127i8;
    let xu8: u8 = 255;
    println!("xi8: {}, xu8: {}", xi8, xu8);
    let xi16 = 127i16;
    let xu16: u16 = 255;
    println!("xi16: {}, xu16: {}", xi16, xu16);
    let xi32 = 127i32;
    let xu32: u32 = 255;
    println!("xi32: {}, xu32: {}", xi32, xu32);
    let xi64 = 127i64;
    let xu64: u64 = 25_5;
    println!("xi64: {}, xu64: {}", xi64, xu64);
    let xi128 = 127i128;
    let xu128: u128 = 100_100;
    println!("xi128: {}, xu128: {}", xi128, xu128);
    let x_isize = 127isize;
    let x_usize: usize = 255;
    println!("xisize: {}, xusize: {}", x_isize, x_usize);

    let x_decimal = 98_222;
    println!("x_decimal: {}", x_decimal);
    let x_hex = 0xff;
    println!("x_hex: {}", x_hex);
    let x_octal = 0o77;
    println!("x_octal: {}", x_octal);
    let x_binary = 0b1111_0000;
    println!("x_binary: {}", x_binary);
    let x_byte = b'A';
    println!("x_byte: {}", x_byte);

    let f = 2.0;
    println!("f {}", f);
    let f: f32 = 3.0;
    println!("f {}", f);
}
