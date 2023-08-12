fn main() {
    // Floating-Point Types 浮点类型
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // =================================================================
    // Numeric Operations 数值运算
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // =================================================================
    // The Boolean Type 布尔类型
    let t = true;

    let f: bool = false; // with explicit type annotation

    // =================================================================
    // The Character Type 字符类型
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // =================================================================
    // The Tuple Type 元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // =================================================================
    // The Array Type 数组类型
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //  This is the same as writing let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    // Accessing Array Elements 访问数组元素
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // Invalid Array Element Access 非法数组元素访问
    //     let a = [1, 2, 3, 4, 5];

    //     println!("Please enter an array index.");

    //     let mut index = String::new();

    //     io::stdin()
    //         .read_line(&mut index)
    //         .expect("Failed to read line");

    //     let index: usize = index
    //         .trim()
    //         .parse()
    //         .expect("Index entered was not a number");

    //     let element = a[index];

    //     println!("The value of the element at index {index} is: {element}");
    //     thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
