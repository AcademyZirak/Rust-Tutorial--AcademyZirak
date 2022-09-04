fn main() {
    println!("Hello, world!");

    let my_number_1 = 21;
    // if we don't define a type rust compiler will assign a suitable type to it.

    let my_number_2: i32 = 22;

    // in Rust we have sighed numbers(i) and unsigned numbers(u) (zero and positive numbers)

    // u8 means unsigned number which can be made with 8bit of RAM.

    // Accessing methods of a type in Rust using ::

    println!("u8: Min {} ---> Max {}", u8::min_value(), u8::max_value());

    println!("i8: Min {} ---> Max {}", i8::min_value(), i8::max_value());

    println!(
        "i32: Min {} ---> Max {}",
        i32::min_value(),
        i32::max_value()
    );

    println!(
        "i64: Min {} ---> Max {}",
        i64::min_value(),
        i64::max_value()
    );

    println!(
        "i128: Min {} ---> Max {}",
        i128::min_value(),
        i128::max_value()
    );

    // isize depends on your system architecture.
    // if your system is 64bit based isize means i64 and usize means u64
    println!(
        "isize: Min {} ---> Max {}",
        isize::min_value(),
        isize::max_value()
    );

    // when do we use this types?
    // for example if the variable is the age of a user u4 is not enough and u8 is good.
    // default number type in rust is i32.

    // using n bits of the RAM we can store 2^n numbers.

    // if we store unsigned numbers we can store from 0 to 2^n-1
    // if we store signed numbers we can store from -2^(n-1) to 2^(n-1)-1
}
