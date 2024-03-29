fn main() {
    // integers
    let x: i32 = -1000;
    println!("x: {}", x);
    let x = -1000i32;
    let y: u64 = 100;
    println!("y: {}", y);
    let y = 100u64;
    let large_number = 100_000_000;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("large_number: {}", large_number);

    dbg!(x, y);

    // floats
    let xf: f32 = -1.2345;
    println!("xf: {}", xf);
    let xf = -1.2345_f32;
    println!("xf: {}", xf);

    // complex numbers
    // not part of rusts standard library
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);
    println!("complex_integer: {}", complex_integer);
    println!("complex_float: {}", complex_float);


    // binary literals
    let x: u8 = 0b1010_1010;
    let y: u8 = 0b0101_0101;
    dbg!(x | y);

    // hex
    let x: u8 = 0xAF;
    dbg!(x);
    let largex = 0xABCD_EF01_u32;
    dbg!(largex);

    // boolean
    let yes = true;
    let no = false;

    dbg!(yes);
    dbg!(no);
}