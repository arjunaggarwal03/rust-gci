//Arjun Aggarwal
//Morse Code Translation in Rust
//12-24-19

#[macro_use] extern crate text_io;
use std::collections::HashMap;

fn main() {
    println!("\n1) English to Morse Code");
    println!("2) Morse Code to English");
    println!("\nSelect one option (enter the number):");
    let i: i32 = read!();

    if i == 1 {
        println!("\nEnter text:");
    }
    else{
        println!("\nEnter code (put a space between characters and >1 spaces between words):");
    }

    let sen: String = read!("{}\n");
    let senlc = sen.to_lowercase();

    let trns_a: HashMap<char, &str> = [('a',".-"),('b',"-..."),('c',"-.-."),('d',"-.."),('e',"."),
                                    ('f',"..-."),('g',"--."),('h',"...."),('i',".."),('j',".---"),
                                    ('k',"-.-"),('l',".-.."),('m',"--"),('n',"-."),('o',"---"),
                                    ('p',".--."),('q',"--.-"),('r',".-."),('s',"..."),('t',"-"),
                                    ('u',"..-"),('v',"...-"),('w',".--"),('x',"-..-"),('y',"-.--"),
                                    ('z',"--.."),(' ',"    ")].iter().cloned().collect();

    let trns_b: HashMap<&str, char> = [(".-",'a'),("-...",'b'),("-.-.",'c'),("-..",'d'),(".",'e'),
                                    ("..-.",'f'),("--.",'g'),("....",'h'),("..",'i'),(".---",'j'),
                                    ("-.-",'k'),(".-..",'l'),("--",'m'),("-.",'n'),("---",'o'),
                                    (".--.",'p'),("--.-",'q'),(".-.",'r'),("...",'s'),("-",'t'),
                                    ("..-",'u'),("...-",'v'),(".--",'w'),("-..-",'x'),("-.--",'y'),
                                    ("--..",'z')].iter().cloned().collect();

    let mut ret: String = "".to_string();

    if i == 1{
        for x in senlc.chars(){
            ret += &(trns_a[&x].to_owned() + " ");
        }
    }
    else if i == 2{
        let split = senlc.split(" ");
        let mut spaces: i32 = 0;
        let vec: Vec<&str> = split.collect();
        for x in vec {
            if trns_b.contains_key(x) {
                spaces = 0;
                ret += &trns_b[x].to_string();
            }
            if trns_b.contains_key(x) == false {
                spaces += 1;
            }
            if trns_b.contains_key(x) == false && spaces <= 1{
                ret += " ";
            }
        }
    }

    println!("\nOutput: {}",ret);

}
