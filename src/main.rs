fn main() {
    loop {
        print!(">");
        // Flushing the stdout buffer to make sure the prompt is printed
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        // Running the inputted command in a new process (will not work on Windows without going through powershell or cmd)
        if let Ok(mut curr_process) = std::process::Command::new(command).spawn() {
            // Waiting for the process to finish
            curr_process.wait().expect("Failed to wait on child");
        } else {
            println!("Command not found");
        }
    }
}
