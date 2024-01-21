use std::io;

// Define a generic function to get user input
pub fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue
        }
    }
}