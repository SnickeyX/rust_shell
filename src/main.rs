fn main() {
    loop {
        print!(">");
        // Flushing the stdout buffer to make sure the prompt is printed
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut command_with_args = input.trim().split_whitespace();
        let command = command_with_args.next().unwrap();
        let args = command_with_args;
        // To handle shell builtins - this allows us change the state of the shell rather than the specific process handling the command
        match command {
            "cd" => {
                let path = args.peekable().peek().map_or("/", |x| *x);
                if let Err(e) = std::env::set_current_dir(path) {
                    println!("Failed: {}", e);
                }
            }
            "exit" => {
                std::process::exit(0);
            }
            command => {
                // Running the inputted command in a new process (will not work on Windows without going through powershell or cmd)
                if let Ok(mut curr_process) = std::process::Command::new(command).args(args).spawn()
                {
                    // Waiting for the process to finish
                    curr_process.wait().expect("Failed to wait on child");
                } else {
                    println!("Command not found");
                }
            }
        }
    }
}
