//Arjun Aggarwal
//Armstrong Numbers in Rust
//12-24-19

#[macro_use] extern crate text_io;

use std::convert::TryInto;

fn main() {
    println!("Input number: ");
    let i: i32 = read!();

    let temp = i.to_string();
    let size: u32 = temp.len().try_into().unwrap();
    let mut ret = 0;

    let mut b = "".to_owned();

    for x in temp.chars(){
        let z: i32 = (x.to_string()).parse::<i32>().unwrap().pow(size);
        ret += z;
        b += &(x.to_string() + "^" + &size.to_string() + " + ");
    }

    let mut sent = String::new();

    if i != ret{
        sent = temp + " is not an Armstrong number because ";
    }
    else{
        sent = temp + " is an Armstrong number because ";
    }

    b.truncate(b.len()-2);
    let c = "= ".to_owned() + &ret.to_string();
    let fin = sent + &b + &c;
    println!("{}",fin);
}
