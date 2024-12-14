use std::io::Write;
use std::{fs, io};

pub fn log(message: &str, kind: &str, newline: bool) {
    match kind {
        "INFO" => {
            if newline {
                println!("\x1b[90m[ \x1b[34m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            } else {
                print!("\x1b[90m[ \x1b[34m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            }
        }
        "ERROR" => {
            if newline {
                eprintln!("\x1b[90m[ \x1b[31m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            } else {
                eprint!("\x1b[90m[ \x1b[31m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            }
        }
        "WARNING" => {
            if newline {
                eprintln!("\x1b[90m[ \x1b[33m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            } else {
                eprint!("\x1b[90m[ \x1b[33m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            }
        }
        "PROMPT" => {
            if newline {
                println!("\x1b[90m[ \x1b[35m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            } else {
                print!("\x1b[90m[ \x1b[35m{}\x1b[90m ]\x1b[0m {}", kind.to_uppercase(), message);
            }
        }
        "HEART" => {
            // It's off center in Konsole, but it's the thought that counts.
            if newline {
                println!("\x1b[90m[ \x1b[31m❤️\x1b[90m ]\x1b[0m {}", message);
            } else {
                print!("\x1b[90m[ \x1b[31m❤️\x1b[90m ]\x1b[0m {}", message);
            }
        }
        _ => {
            if newline {
                println!("\x1b[90m[ {}\x1b[0m ] {}", kind.to_uppercase(), message);
            } else {
                print!("\x1b[90m[ {}\x1b[0m ] {}", kind.to_uppercase(), message);
            }
        }
    }
}

pub fn load_file(file: &str) -> io::Result<String> {
    fs::read_to_string(file)
}

pub fn create_tempfile(name: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn delete_tempfile(name: &str) -> io::Result<()> {
    fs::remove_file(name)
}

pub fn get_sys_prompt() -> String {
    String::from(r#"
    YOU ARE A JAVA CODE OBFUSCATOR. YOUR GOAL IS TO TAKE INPUT JAVA CODE AND OUTPUT AN OBFUSCATED VERSION. NOTE THAT
    OBFUSCATING NUMBERS WITH MATH TYPICALLY WONT WORK AS THEY ARE SOLVED DURING COMPILATION. YOU WILL ONLY REPLY WITH
    THE OBFUSCATED CODE AND NO OTHER TEXT.

    FOLLOW THESE RULES:
    1. YOU SHOULD OBFUSCATE STRINGS, EXCEPTIONS, AND METHOD CALLS WHILST STILL KEEPING THE SAME
    FUNCTIONALITY AND RESULT, YOU CAN DO THIS BY FOR EXAMPLE TURNING THE STRING INTO A BYTE ARRAY.

    2. YOU SHOULD ADD UNNECESSARY IF STATEMENTS AND WHILE LOOPS THAT BREAK WHILST
    STILL KEEPING THE SAME FUNCTIONALITY AND RESULT. EVICERATE THE CONTROL FLOW.

    3. YOU SHOULD OBFUSCATE AND RANDOMIZE VARIABLE NAMES WHILST STILL KEEPING
    THE SAME FUNCTIONALITY AND RESULT.

    4. YOU SHOULD OBFUSCATE NUMBERS BY INTRODUCING VARIABLES AND USING SIMPLISTIC MATH FUNCTIONS
    TO END UP WITH THE SAME RESULT AND FUNCTIONALITY.

    5. DO NOT USE MARKDOWN OR ANY OTHER FORMATTING IN YOUR RESPONSE.

    6. REPLY IN PLAIN TEXT.

    7. DO REFORMAT THE FILE.

    8. RENAME MAIN CLASS TO 'AIfuscatorMyBelovedXoXo'.

    9. YOU SHOULD MAKE IT AS CONVOLUTED AS POSSIBLE WHILST STILL KEEPING THE SAME AND RESULT.

    10. YOU SHOULD MAKE IT AS CONFUSING AS POSSIBLE WHILST STILL KEEPING THE SAME AND RESULT.
    "#)
}
