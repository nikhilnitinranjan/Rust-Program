fn main() {
    //variable
    let mut i = 5;
    println!("The value of i is: {}", i);

    i = 6;
    println!("The value of i is: {}", i);

    //shadowing
    let x = 15;
    println!("The value of x is: {}", x);

    let x = " hello nikhil";
    println!("The value of x is: {}", x);


    //constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

}
