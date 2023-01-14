use std::process::Child;

fn main() {
    loop {
        print!(">");
        // Flushing the stdout buffer to make sure the prompt is printed
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_command = None;
        while let Some(command) = commands.next() {
            let mut command_with_args = command.split_whitespace();
            let command = command_with_args.next().unwrap();
            let args = command_with_args;
            // To handle shell builtins - this allows us change the state of the shell rather than the specific process handling the command
            match command {
                "cd" => {
                    let path = args.peekable().peek().map_or("/", |x| *x);
                    if let Err(e) = std::env::set_current_dir(path) {
                        println!("Failed: {}", e);
                    }
                    prev_command = None;
                }
                "exit" => {
                    std::process::exit(0);
                }
                command => {
                    // Get the stdout of the previous command to use as the stdin of the curr command
                    let stdin = prev_command
                        .map_or(std::process::Stdio::inherit(), |output: Child| {
                            std::process::Stdio::from(output.stdout.unwrap())
                        });
                    // If there is another command to run, pipe the stdout of the current command to the stdin of the next command
                    // Otherwise, use the stdout of the current command as the stdout of the shell
                    let stdout = if commands.peek().is_some() {
                        std::process::Stdio::piped()
                    } else {
                        std::process::Stdio::inherit()
                    };
                    // Running the inputted command in a new process (will not work on Windows without going through powershell or cmd)
                    let running_process_output = std::process::Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match running_process_output {
                        Ok(running_process_output) => {
                            prev_command = Some(running_process_output);
                        }
                        Err(e) => {
                            println!("Failed: {}", e);
                            prev_command = None;
                        }
                    }
                }
            }
        }
        // Wait for the last command to finish
        if let Some(mut last_command) = prev_command {
            last_command
                .wait()
                .expect("Failed to wait for child process");
        }
    }
}
