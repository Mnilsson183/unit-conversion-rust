use std::io;

fn main() {
    println!("what units do you want to convert");
    println!("Format in [X Units->Final Unit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let middle: u32 = 0;

    let input_vector: Vec<char> = input.chars().collect();

    for middle in 0..input_vector.len()-1
    {
        if input_vector[middle] == '-' && input_vector[middle + 1] == '>'
        {
            println!("{}", middle);
            break;
        }
    }

    let pre = vec!['T', 'G', 'M', 'k', 'm', 'μ', 'n', 'p'];
    let pre_value = vec![12, 9, 6, 3, -3, -6, -9, -12];


}
