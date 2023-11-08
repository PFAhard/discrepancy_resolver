#[macro_export]
macro_rules! prompt {
    ($fmt:expr $(, $args:expr)* ; $yes:block $no:block) => {{
        use std::io::{self, Write};

        // Using `print!` to support formatting
        print!($fmt $(, $args)*);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.to_lowercase().trim() {
            "yes" | "y" => $yes,
            "no" | "n" => $no,
            _ => unreachable!("Answer '{}' is not recognized", input.trim()),
        }
    }};
}

#[macro_export]
macro_rules! uni_prompt {
    ($fmt:expr $(, $args:expr)*) => {{
        use std::io::{self, Write};

        // Using `print!` to support formatting
        print!($fmt $(, $args)*);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        input.trim().to_owned()
    }};
}