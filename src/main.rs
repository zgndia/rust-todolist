use std::io;

fn main() {
    
    let mut tasks: Vec<String> = Vec::new();

    loop {
        print_bars();
        print_tasks(tasks.clone());
        print_bars();
        print_commands();
        print_bars();
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line).expect("failed to read a line");

        if bytes == 2 {
            println!("Input is empty, exiting...");
            break;
        }

        let mut inputs = Vec::new();

        for word in line.split_whitespace() {
            inputs.push(word);
        }

        if inputs[0] == "new" {
            // get the rest of the text except for the command then trim
            let mut full = String::new();

            for i in 1..inputs.len() {
                full.push_str(&format!("{} ", inputs[i]));
            }

            full = full.trim().to_string();

            println!("Adding task \"{full}\"!");
            tasks.push(full);
        } else if inputs[0] == "del" || inputs[0] == "complete" {
            let num = inputs[1]; // get input after first space
            let num: usize = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Correct usage: del 1/2/3 (task #)");
                    continue;
                },
            };

            // check if it is in range first
            if num > tasks.len() {
                println!("Task number {num} is not in range, try another number!");
                continue;
            }

            let index = num - 1;
            tasks.remove(index);

            println!("Task #{num} successfully deleted!");
        } else if inputs[0] == "clear" || inputs[0] == "cl" {
            println!("Clearing current list!");
            tasks.clear();
        } else if inputs[0] == "quit" || inputs[0] == "exit" || inputs[0] == "q" {
            println!("Exiting...");
            break;
        } else {
            println!("Invalid command.");
        }
    }
}

fn print_commands() {
    let cmds = [["new \"task\"", "creates a new task"], ["del #", "deletes a task with it's number"], ["clear/cl", "clears the list"], ["exit/quit/q", "exits the program"]];
    let target = 12;

    println!("Commands:");
    
    for cmd in cmds {

        let mut cmd0 = cmd[0].to_string();
        
        let ltrs = cmd0.chars().count();
        if ltrs < target {
            for _ in 0..target - ltrs {
                cmd0 += " ";
            }
        }
        println!("{} => {}", cmd0, cmd[1]);
    }
}

fn print_tasks(v: Vec<String>) {

    let mut count = 1;

    println!("Current Tasks:");
    
    if v.len() == 0 {
        println!("Nothing to do!");
    } else {
        for task in v {
            println!("{count}. {task}");
            count += 1;
        }
    }
}

fn print_bars() { // prints 50 dashes '-'
    println!("--------------------------------------------------");
}