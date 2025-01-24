use std::io;

fn main() {
    let mut input = String::new();
    let mut result: i32 = 0;

    loop {
        io::stdin().read_line(&mut input).expect("Error reading input");

        if input.trim() == "Q" { 
            break; 
        }

        result += input.trim().parse::<i32>().expect("Error parsing input");
        
        input.clear();
    }

    println!("{:?}", result)
        
}
