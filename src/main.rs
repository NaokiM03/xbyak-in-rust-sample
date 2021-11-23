mod xbyak;
use xbyak::{add, return_same_value};

fn main() {
    println!("{}", return_same_value(42));
    println!("{}", add(2, 3));
}
