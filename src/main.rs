use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(());
    }

    let filename = &args[1];

    let mut lines: Vec<String> = Vec::new();
    if let Ok(file) = fs::File::open(filename) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            lines.push(line?);
        }
    }

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim().to_lowercase();
        if command == "q" || command == "quit" {
            break;
        } else if command == "p" || command == "print" {
            for line in &lines {
                println!("{}", line);
            }
        } else if command.starts_with("a ") || command.starts_with("append ") {
            let content = command.trim_start_matches("a ").trim_start_matches("append ");
            lines.push(content.to_string());
        } else if command.starts_with("i ") || command.starts_with("insert ") {
            let parts: Vec<&str> = command.splitn(3, ' ').collect();
            if parts.len() < 3 {
                println!("Invalid command");
                continue;
            }
            let line_num = parts[1].parse::<usize>().unwrap_or(0);
            let content = parts[2];
            lines.insert(line_num, content.to_string());
        } else if command.starts_with("d ") || command.starts_with("delete ") {
            let line_num = command.trim_start_matches("d ").trim_start_matches("delete ").parse::<usize>().unwrap_or(0);
            if line_num >= lines.len() {
                println!("Invalid line number");
                continue;
            }
            lines.remove(line_num);
        } else {
            println!("Invalid command");
        }
    }

    let mut file = fs::File::create(filename)?;
    for line in &lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
