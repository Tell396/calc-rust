use std::io;

fn calc(a: i64, b: i64) {
    println!("The result is: {}", a + b);
}

fn main() {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    let mut input_line2 = String::new();
    io::stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read line");
    
    let x: i64 = input_line
        .trim()
        .parse()
        .expect("Input not an integer");

    let y: i64 = input_line2
        .trim()
        .parse()
        .expect("Input not an integer");
    
    calc(x, y);
}
