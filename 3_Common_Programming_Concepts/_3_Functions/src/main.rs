fn main() {
    println!("Hello, world!");
    // Functions
    // another_function();
    // Parameters 参数
    // rust没重载
    another_function(5);
}

// Functions
// fn another_function() {
//     println!("Another function.");
// }

// Parameters 参数
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
