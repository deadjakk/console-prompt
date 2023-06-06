use console_prompt::{Command, command_loop, DynamicContext};
use std::error::Error;
use std::any::Any;

fn yes(_args: &[&str], _context: Option<&Box<dyn Any>>)->Result<String, Box<dyn Error>>{
    Ok(format!("You responded with a yes"))
}

fn no(_args: &[&str], _context: Option<&Box<dyn Any>>)->Result<String, Box<dyn Error>>{
    Ok(format!("You responded with a no"))
}


fn change(_args: &[&str], context: &mut DynamicContext)->Result<String, Box<dyn Error>>{
    match context.get_mut::<String>() {
        Some(mut_ref) => {
            *mut_ref = _args[0].to_string();
            return Ok(format!("you changed their name to {}", *mut_ref));
        },
        None => return Ok("you are not in a conversation with a person".to_string()),
    }
}

fn hello(_args: &[&str], context: &mut DynamicContext)->Result<String, Box<dyn Error>>{
    match context.get::<String>() {
        None => { 
            return Ok("you are not in a conversation with a person".to_string()) 
        },
        Some(name ) => {
            return Ok(format!("You said hello to {name} I guess"));
        }
    }
}

// test function that demonstrates calling a command_loop sub call
// to provide a nested state
fn converse(args: &[&str], _context: &mut DynamicContext)->Result<String, Box<dyn Error>>{
    if args.len() == 0 {
        return Ok("no name provided".to_string());
    }
    println!("interacting with: {}", args[0]);
    let commands = vec![
        Command{command: "hello", func: hello, help_output: "hello - say hello"},
        Command{command: "change", func: change, help_output: "change <name> - change the name of the person with whom you're speaking"},
        //Command{command: "yes", func: yes, help_output: "yes - reply yes"},
        //Command{command: "no", func: no, help_output: "no - reply no"},
    ];

    // let mut name: Option<Box<dyn Any>> = Some(Box::new(args[0].to_string()));
    let mut context = DynamicContext::new();
    context.set(args[0].to_string());
    // passing the arguments for the converse commands as context
    // to the commands in the next command loop for reference
    if let Err(e) = command_loop(&commands, &mut context){
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
    if let Err(e) = command_loop(&commands, &mut DynamicContext::new()){
        eprintln!("error running command loop: {}", e.to_string());
    }
}
