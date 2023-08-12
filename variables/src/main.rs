fn main() {
    // let mut x = 5;
    // println!("The value x is: {x}");
    // x = 6;
    // println!("The value x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;
    {
        let x = x*2;
        println!("The value x in the inner scope is: {x}");
    }
    println!("The value x is: {x}");
}
