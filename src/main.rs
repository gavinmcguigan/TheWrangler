use std::io::Write;
use std::process::{Command, Stdio};

fn _only_ag() {
    // Create ag command
    let mut ag = Command::new("ag");
    ag.arg("-g").arg("$").arg("/home/gavin/github/");

    // Execute ag command and capture its output
    let ag_out = ag.output().unwrap().stdout;

    // Create fzf command and set stdin and stdout to be piped
    let mut fzf = Command::new("fzf");
    fzf.stdin(Stdio::piped());
    fzf.stdout(Stdio::piped());

    let mut fzf_process = fzf.spawn().unwrap();
    fzf_process
        .stdin
        .take()
        .unwrap()
        .write_all(&ag_out)
        .unwrap();

    let fzf_process_output = fzf_process.wait_with_output().unwrap().stdout;
    let selected_file = String::from_utf8(fzf_process_output)
        .expect("Invalid UTF-8 sequence")
        .replace('\0', "")
        .replace('\n', "");
    if selected_file.is_empty() {
        return;
    }

    println!("Selected file: '{}'", selected_file);

    let mut vim = Command::new("vim");
    vim.arg(selected_file);
    vim.status().expect("Failed to execute vim");
    // let vim_process = vim.spawn().expect("Failed to start vim");
}

fn main() {
    _only_ag();
}
