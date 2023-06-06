use console_prompt::{Command, command_loop};
use std::error::Error;
use std::any::Any;

fn yes(_args: &[&str], _context: Option<&Box<dyn Any>>)->Result<String, Box<dyn Error>>{
    Ok(format!("You responded with a yes"))
}

fn no(_args: &[&str], _context: Option<&Box<dyn Any>>)->Result<String, Box<dyn Error>>{
    Ok(format!("You responded with a no"))
}


fn change(_args: &[&str], context: &mut Option<Box<dyn Any>>)->Result<String, Box<dyn Error>>{
    match context {
        None => { return Ok("you are not in a conversation with a person".to_string()) },
        Some(data) => {
            if let Some(name_val) = data.downcast_mut::<String>(){
                *name_val = _args[0].to_string();
            }
            if let Some(name) = data.downcast_ref::<String>(){
                return Ok(format!("You changed their name to {name}"));
            } 
            return Ok(format!("You could not get a name to change"));
        }
    }
}

fn hello(_args: &[&str], context: &mut Option<Box<dyn Any>>)->Result<String, Box<dyn Error>>{
    match context {
        None => { return Ok("you are not in a conversation with a person".to_string()) },
        Some(data) => {
            let name_op = data.downcast_ref::<String>();
            return Ok(format!("You said hello to {} I guess", name_op.unwrap() ));
        }
    }
}

// test function that demonstrates calling a command_loop sub call
// to provide a nested state
fn converse(args: &[&str], _context: &mut Option<Box<dyn Any>>)->Result<String, Box<dyn Error>>{
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

    let mut name: Option<Box<dyn Any>> = Some(Box::new(args[0].to_string()));
    // passing the arguments for the converse commands as context
    // to the commands in the next command loop for reference
    if let Err(e) = command_loop(&commands, &mut name){
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
    if let Err(e) = command_loop(&commands, &mut None){
        eprintln!("error running command loop: {}", e.to_string());
    }
}
