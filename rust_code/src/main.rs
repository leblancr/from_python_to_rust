fn main() {

    let x = 1;
    println!("x: {}", x);
    // binds x again, shadowing the old one from above
    let x = 'i';
    println!("x: {}", x);

    // declare, initialize
    let something;
    let x = 5;
    //println!("x, something: {}, {}", x, something);  // uninitialized
    something = x * 5;
    println!("x, something: {}, {}", x, something);

    // Mutability
    let mut y = 0;
    y = y * 2 + x;
    dbg!(y);

    const BLAH: i32 = 42;
    y *= BLAH;
    dbg!(y);
}