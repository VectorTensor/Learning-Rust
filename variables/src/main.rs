use std::io;
fn main() {

    let a: [i32; 5] = [1,2,3,43,5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of element at index {index} is :: {element}");



}
