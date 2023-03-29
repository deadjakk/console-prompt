use console_prompt::{Command, command_loop};
use std::error::Error;

fn yes(_args: &[&str], _context: Option<&[&str]>)->Result<String, Box<dyn Error>>{
    Ok(format!("You responded with a yes"))
}

fn no(_args: &[&str], _context: Option<&[&str]>)->Result<String, Box<dyn Error>>{
    Ok(format!("You responded with a no"))
}

fn hello(_args: &[&str], context: Option<&[&str]>)->Result<String, Box<dyn Error>>{
    if let None = context {
        return Ok("you are not in a conversation with a person".to_string());
    }
    Ok(format!("You said hello to {} I guess", context.unwrap()[0]))
}

// test function that demonstrates calling a command_loop sub call
// to provide a nested state
fn converse(args: &[&str], _context: Option<&[&str]>)->Result<String, Box<dyn Error>>{
    if args.len() == 0 {
        return Ok("no name provided".to_string());
    }
    println!("interacting with: {}", args[0]);
    let commands = vec![
        Command{command: "hello", func: hello, help_output: "hello - say hello"},
        Command{command: "yes", func: yes, help_output: "yes - reply yes"},
        Command{command: "no", func: no, help_output: "no - reply no"},
    ];

    // passing the arguments for the converse commands as context
    // to the commands in the next command loop for reference
    if let Err(e) = command_loop(&commands, Some(args)){
        eprintln!("error running interact command loop: {}", e);
    }
    Ok(String::new())
}

fn main(){
    let commands = vec![
        Command{
            command: "converse",
            func: converse,
            help_output: "converse <name> - interact with a person"
        },
    ];

    // start the command loop with the provided commands
    if let Err(e) = command_loop(&commands, None){
        eprintln!("error running command loop: {}", e.to_string());
    }
}
