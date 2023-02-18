use std::io;

fn main() {
    println!("what units do you want to convert");
    println!("Format in [X Units->Final Unit]");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let middle: usize = 0;

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

    let mut input_units:usize = 0;
    let mut input_value:usize = 0;
    let mut output_units:usize = 0;
    let mut output_value:usize = 0;

    for i in (0..pre.len())
    {
        if input_vector[0] == pre[i]
        {
            input_units = i;
        }
        else
        {
            if input_vector[0] == pre[pre.len() - 1]
            {
                println!("The first unit value is either not supported or not a unit");
            }
        }
    }
    for i in (0..pre.len())
    {
        if input_vector[middle + 2] == pre[i]
        {
            output_units = i;
        }
        else
        {
            if input_vector[0] == pre[pre.len() -  1]
            {
                println!("The second unit value is either not supported or not a unit");
            }
        }
    }
    println!("{input_units}");
    println!("{output_units}");
}
