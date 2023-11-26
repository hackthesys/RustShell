use std::{process::{Command, Stdio}, io::{self, Write, Stdin, Stdout}};

fn main()
{
    exeComd();
}

  fn exeComd()
  {
    printBanner();
    loop {
        print!("PS_>: ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Bye!!!");
            break;
        }
        // Execute the command using PowerShell
        let commandOutput = Command::new("powershell")
            .arg("-Command")
            .arg(&input)
            .output()
            .expect("Failed to execute command.");

        
        // Print the output of the command
        println!("{}", String::from_utf8_lossy(&commandOutput.stdout));

        // Handle errors if the command execution fails
        if !commandOutput.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&commandOutput.stderr));
        }
    }
}

fn printBanner()
 {

    let banner = r#"
    ___________
    < RustShell >
     -----------
       \         ,        ,
        \       /(        )`
         \      \ \___   / |
                /- _  `-/  '
               (/\/ \ \   /\
               / /   | `    \
               O O   ) /    |
               `-^--'`<     '
              (_.)  _  )   /
               `.___/`    /
                 `-----' /
    <----.     __ / __   \
    <----|====O)))==) \) /====
    <----'    `--' `.__,' \
                 |        |
                  \       /
            ______( (_  / \______
          ,'  ,-----'   |        \
          `--{__________)        \/
"#;
    print!("{banner}");
 }
