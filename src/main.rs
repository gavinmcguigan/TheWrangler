use std::io::Write;
use std::process::{Command, Stdio};
use dirs;

fn execute_ag_command() -> Vec<u8> {
    // Resolve the home directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let home_dir_str = home_dir.to_str().expect("Failed to convert home directory to string");

fn execute_ag_command() -> Vec<u8> {
    // Create ag command
    let mut ag = Command::new("ag");
    ag.arg("-g").arg("$").arg(home_dir_str);

    // Execute ag command and capture its output
    match ag.output() {
        Ok(output) => {
            if !output.status.success() {
                std::process::exit(1);
            }
            return output.stdout;
        }
        Err(_e) => {
            eprintln!("Error executing command: 'ag'");
            std::process::exit(1);
        }
    }
}

fn execute_fzf_command(ag_out: Vec<u8>) -> String {
    // Create fzf command and set stdin and stdout to be piped
    let mut fzf = Command::new("fzf");
    fzf.arg("--preview").arg("sudo -E batcat --style=full --color=always {}").arg("--preview-window").arg("top:80%:wrap");
    fzf.stdin(Stdio::piped());
    fzf.stdout(Stdio::piped());

    match fzf.spawn() {
        Ok(mut process) => match process.stdin.take() {
            Some(mut stdin) => match stdin.write_all(&ag_out) {
                Ok(_) => {
                    let output = match process.wait_with_output() {
                        Ok(output) => output.stdout,
                        Err(_e) => {
                            std::process::exit(1);
                        }
                    };
                    let user_choice = String::from_utf8(output)
                        .expect("Invalid UTF-8 sequence")
                        .replace('\0', "")
                        .replace('\n', "");
                    if user_choice.is_empty() {
                        std::process::exit(1);
                    }
                    return user_choice;
                }
                Err(_e) => {
                    std::process::exit(1);
                }
            },
            None => {
                std::process::exit(1);
            }
        },
        Err(_e) => {
            eprintln!("Error executing command: 'fzf'.");
            std::process::exit(1);
        }
    };
}

fn execute_vim_command(selected_file_path: String) {
    let mut vim = Command::new("vim");
    vim.arg(selected_file_path);
    match vim.status() {
        Ok(status) => {
            if !status.success() {
                std::process::exit(1);
            }
        }
        Err(_e) => {
            eprintln!("Error executing command: 'vim'");
            std::process::exit(1);
        }
    }
}

fn main() {
    let ag_out = execute_ag_command();
    let selected_file_path = execute_fzf_command(ag_out);
    execute_vim_command(selected_file_path);
}
