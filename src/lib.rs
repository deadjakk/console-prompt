use crossterm::{
    terminal,
    cursor,
    ExecutableCommand,
    QueueableCommand,
    csi,
    Command as ctCommand,
};
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;
use rustyline::error::ReadlineError;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SetScrollingRegion(pub u16, pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetScrollingAll();

#[derive(Debug)]
pub enum CrosstermError {
  UnimplementedInWindows,
}

impl std::error::Error for CrosstermError {}

impl fmt::Display for CrosstermError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CrosstermError::UnimplementedInWindows => write!(f, 
          "This command is unimplemented for Windows"),
    }
  }
}


/// A command that restricts terminal output scrolling within the given 
/// starting and ending rows.
impl ctCommand for SetScrollingRegion {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("{};{}r"), self.0, self.1)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), CrosstermError> {
        Err(CrosstermError::UnimplementedInWindows)
    }
}

/// Enables scrolling for the entire screen.  
/// This is called after running SetScrollingRegion
/// to re-enable terminal scrolling for the entire screen.  
impl ctCommand for SetScrollingAll {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, csi!("r"))
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<(), CrosstermError> {
        Err(CrosstermError::UnimplementedInWindows)
    }
}


/// Runs the actual command loop, providing readline output via rustyline.
/// The context arg allows for the passing of additional 'context' information
/// to maintain a state during subcalls if needed.
pub fn command_loop(commands: &Vec<Command>, context: &mut Option<Box<dyn Any>>) -> Result<(), Box<dyn Error>>{
    setup_screen()?;

    println!("info: type 'help' to for a list of commands");
    let help_str = build_help_str(&commands);
    loop { // command loop
        if let Err(err) = setup_screen(){
            eprintln!("error during screen setup: {}", err.to_string());
        }
        let mut rl = rustyline::Editor::<()>::new().unwrap();
        match rl.readline(">> "){
            Ok(line)=>{
                if line.is_empty(){continue}

                let mut input_split = line.split(' ').collect::<Vec<_>>(); // TODO needs better
                                                                           // tokenization
                let input_command = input_split.remove(0);
                let input_args = &input_split;
                
                // check for the help command
                if input_command.eq("help") || input_command.eq("?") {
                    write_output(help_str.clone(), None)?;
                    continue;
                }
                if input_command.eq("exit") {
                    break;
                }

                for cmd in commands.into_iter().filter(|cmd| cmd.command.eq(input_command)) {
                    let output = (cmd.func)(&input_args, context);
                    match output {
                        Err(err) => eprintln!("error executing '{}': {}", input_command, err.to_string()),
                        Ok(output_str) => write_output(output_str, None).expect("error writing output"),
                    }
                }
            },
            Err(ReadlineError::Interrupted) => std::process::exit(0),
            Err(err)=>{
                eprintln!("error during readline: {}",err.to_string());
                break;
            }
        }
    }

    Ok(())
}

fn build_help_str(commands: &Vec<Command>) -> String {
    let mut help_output = String::from("---help output------------\n");
    commands.into_iter().for_each(|cmd| help_output.push_str(&format!("{}\n", cmd.help_output)));
    help_output.push_str("exit - exit the current prompt");
    help_output
}


/// This function will print a line to the screen, one line above the
/// bottom-most row of the terminal.  
/// Optionally, a prefix can be provided in case you would like to add
/// additional context to the output line.
pub fn write_output(output: String, prefix: Option<String>)->Result<(),Box<dyn Error>>{
    let mut sout = io::stdout().lock();

    // return order (columns, rows)
    let size = crossterm::terminal::size()?;
    let stdout_end = size.1-1;
    let mut final_output = String::new();

    // add the prefix to the output if it was provided
    if let Some(line) = prefix {
        final_output.push_str(line.as_str());
        final_output.push_str(": ");
    }

    final_output.push_str(output.as_str());

    sout.queue(cursor::SavePosition)?;

    // restrict scrolling to a specific area of the screen
    // this is run every time in case the screen size changes at some point
    sout.queue(SetScrollingRegion(1,stdout_end))?
        .queue(terminal::ScrollUp(1))? 
        .queue(cursor::MoveTo(0, stdout_end-1))?; // move to the line right above stdin

    print!("{}", final_output);
    sout.queue(SetScrollingAll())?
        .queue(cursor::RestorePosition)?;
    sout.flush()?;
    Ok(())
}

/// Sets the cursor location to the bottom-most row and the column to 1.  
/// This gets run automatically in command_loop().
pub fn setup_screen()->Result<(),Box<dyn Error>>{
    let size = crossterm::terminal::size()?;
    let mut sout = std::io::stdout().lock();
    sout.execute(cursor::MoveToRow(size.1))?;
    Ok(())
}

pub struct Command<'r> {
    pub command: &'r str,
    pub func: fn(&[&str], &mut Option<Box<dyn Any>>)->Result<String, Box<dyn Error>>,
    pub help_output: &'r str,
}
