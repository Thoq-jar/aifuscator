use std::io::Write;
use aifuscator::{create_tempfile, delete_tempfile, get_sys_prompt, load_file, log};
use openai_api_rust::chat::*;
use openai_api_rust::*;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        log("Usage: <executable> <java_file>", "ERROR", true);
        process::exit(1);
    }
    let program_input = &args[1];
    log("Welcome to AIfuscator!", "INFO", true);

    log("Setting up environment...", "INFO", true);
    if std::path::Path::new("AIfuscatorMyBelovedXoXo.class").exists() {
        if let Err(err) = delete_tempfile("AIfuscatorMyBelovedXoXo.class") {
            log(&format!("Failed to delete existing class file: {}", err), "ERROR", true);
        }
    }

    log("Loading your Java program...", "INFO", true);
    let raw_code = match load_file(program_input) {
        Ok(content) => content,
        Err(err) => {
            log(&format!("Failed to load the program: {}!", err), "ERROR", true);
            process::exit(1);
        }
    };

    let system_prompt = get_sys_prompt();
    let prompt = format!("{}", raw_code);
    log(&format!("Obfuscating {}...", program_input), "INFO", true);

    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-4o-mini".to_string(),
        max_tokens: Some(3278),
        temperature: Some(0f32),
        top_p: Some(0_f32),
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![
            Message { role: Role::System, content: system_prompt.to_string() },
            Message { role: Role::User, content: prompt.to_string() },
        ],
    };

    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();

    log("Processing your program...", "INFO", true);
    let response = message.content.to_string();

    log("Loading obfuscated code into memory...", "INFO", true);
    let processed_code = response;

    log("Creating tempfile...", "INFO", true);
    let tempfile_name = "AIfuscatorMyBelovedXoXo.java";
    if let Err(err) = create_tempfile(tempfile_name, &processed_code) {
        log(&format!("Failed to save tempfile: {}", err), "ERROR", true);
    }

    log("Compiling program...", "INFO", true);
    let compile = process::Command::new("javac")
        .arg(tempfile_name)
        .output()
        .expect("Failed to compile program.");

    if compile.status.success() {
        log("Program compiled successfully!", "INFO", true);
        delete_tempfile(tempfile_name).unwrap();
    } else {
        log("Failed to compile program.", "ERROR", true);
        log(&format!("Error: {}", String::from_utf8_lossy(&compile.stderr)), "ERROR", true);
        process::exit(1);
    }

    log("Do you want to run the obfuscated program? (y/n): ", "PROMPT", false);
    std::io::stdout().flush().expect("Failed to flush stdout.");

    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input.");
    let user_input = user_input.trim().to_lowercase();

    if user_input == "y" {
        log("Running the obfuscated program...", "INFO", true);
        let run = process::Command::new("java")
            .arg("AIfuscatorMyBelovedXoXo")
            .output()
            .expect("Failed to run the program.");

        if run.status.success() {
            log("Program executed successfully!", "INFO", true);
            log(&format!(
                            "Output:\n{}",
                            String::from_utf8_lossy(&run.stdout)
                        ), "INFO",
                true);
        } else {
            log("Program execution failed.", "ERROR", true);
            log(&format!(
                            "Error:\n{}",
                            String::from_utf8_lossy(&run.stderr)
                        ), "ERROR",
                true);
        }
    } else {
        log("Skipping program execution as per user's input.", "INFO", true);
    }

    log("Thank you for using AIfuscator!", "HEART", true);
    process::exit(0);
}
