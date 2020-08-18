//Arjun Aggarwal
//Leap Years in Rust
//12-26-19

#[macro_use] extern crate text_io;

fn main() {
    println!("Input year: ");
    let i: i32 = read!();

    if i%400 == 0{
        println!("{} is a leap year.",i);
    }
    else if (i%4 == 0) & (i%100 != 0){
        println!("{} is a leap year.",i);
    }
    else{
        println!("{} is not a leap year.",i);
    }
}
