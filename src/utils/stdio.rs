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
            _ => {
                println!("Answer '{}' is not recognized", input.trim());
                unsafe {
                    // Generate a CTRL+C event for the current process group
                    if  winapi::um::wincon::GenerateConsoleCtrlEvent( winapi::um::wincon::CTRL_C_EVENT, 0) == 0 {
                        eprintln!("Failed to generate CTRL+C event.");
                    } else {
                        println!("CTRL+C event generated successfully.");
                    }
                }

            },
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
