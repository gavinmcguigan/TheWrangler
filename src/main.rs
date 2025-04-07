use std::io::Write;
use std::process::{Command, Stdio};

fn execute_ag_command() -> Vec<u8> {
    // Create ag command
    let mut ag = Command::new("sudo");
    ag.arg("ag")
        .arg("--follow")
        .arg("-g")
        .arg("$")
        .arg("/omd/sites");

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
    fzf.arg("--preview")
        .arg("sudo ls -la {} | head -n1; echo ''; sudo -E batcat --style=full --color=always {}")
        .arg("--preview-window")
        .arg("top:80%:wrap")
        .arg("--bind")
        .arg("ctrl-d:preview-page-down,ctrl-u:preview-page-up");
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
