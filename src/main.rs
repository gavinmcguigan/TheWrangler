use std::io::Write;
use std::process::{Command, Stdio};

fn execute_ag_command() -> Vec<u8> {
    // Create ag command
    let mut ag = Command::new("ag");
    ag.arg("-g").arg("$").arg("/home/gavin/github/");

    // Execute ag command and capture its output
    match ag.output() {
        Ok(output) => {
            if !output.status.success() {
                eprintln!("Error executing ag command.");
                std::process::exit(1);
            }
            return output.stdout;
        }
        Err(e) => {
            eprintln!("Error executing ag command: {}", e);
            std::process::exit(1);
        }
    }
}

fn execute_fzf_command(ag_out: Vec<u8>) -> String {
    // Create fzf command and set stdin and stdout to be piped
    let mut fzf = Command::new("fzf");
    fzf.stdin(Stdio::piped());
    fzf.stdout(Stdio::piped());

    match fzf.spawn() {
        Ok(mut process) => {
            match process.stdin.take() {
                Some(mut stdin) => {
                    match stdin.write_all(&ag_out) {
                        Ok(_) => {
                            // fzf process started successfully
                            let output = match process.wait_with_output() {
                                Ok(output) => output.stdout,
                                Err(_e) => {
                                    eprintln!("Error waiting for fzf process.");
                                    std::process::exit(1);
                                }
                            };
                            let user_choice = String::from_utf8(output)
                                .expect("Invalid UTF-8 sequence")
                                .replace('\0', "")
                                .replace('\n', "");
                            if user_choice.is_empty() {
                                eprintln!("No file selected. Exiting.");
                                std::process::exit(1);
                            }
                            return user_choice
                        }
                        Err(e) => {
                            eprintln!("Error writing to fzf stdin: {}", e);
                            std::process::exit(1);
                        }
                    }
                    
                }
                None => {
                    eprintln!("Error getting fzf stdin.");
                    std::process::exit(1);
                }
            }



        }
        Err(_e) => {
            eprintln!("Error starting fzf process.");
            std::process::exit(1);
        }
    };
}

fn execute_vim_command(fzf_result: String) {
    let mut vim = Command::new("vim");
    vim.arg(fzf_result);
    vim.status().expect("Failed to execute vim");
}

fn main() {
    let ag_out = execute_ag_command();
    let fzf_result = execute_fzf_command(ag_out);
    execute_vim_command(fzf_result);
}
