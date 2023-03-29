# console_prompt

A very simple way to make a clean, thread-safe, interactive CLI interface
in rust.

Running example:

![Example GIF](images/demo.gif)


Creating a prompt is as simple as creating a vector of `Command` structs with pointers to your functions and handing them off to `command_loop`:

```
fn main(){
    let commands = vec![
        Command{
            command: "converse",
            func: converse, // <---- pointer to the converse function
            help_output: "converse <name> - interact with a person"
        },
    ];

    // start the command loop with the provided commands
    if let Err(e) = command_loop(&commands, None){
        eprintln!("error running command loop: {}", e.to_string());
    }
}
```

You can also create nested command_loops that maintain state via the `context` argument, see [the conversation example](https://github.com/deadjakk/console-prompt/blob/main/examples/conversation.rs#L34) for more info.

Currently this does not support windows due to limitations with windows terminal.
If someone asks for windows support i'll try and figure something out.
