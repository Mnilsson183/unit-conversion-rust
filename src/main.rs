use std::io;

fn main() {
    println!("what units do you want to convert");
    println!("Format in [X -> mX]");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    loop
    {
        println!("Hello");
        break;
    }
}
