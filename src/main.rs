fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let command = input.trim();
    // Running the inputted command in a new process (will not work on Windows without going through powershell or cmd)
    std::process::Command::new(command).spawn().unwrap();
}
