use itertools::Itertools;
use std::fs;
use std::io::{self, Write};
use std::process::{exit, Command};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter)]
enum ContestType {
    Abc,
    Arc,
    Agc,
    Exit,
}
fn parse_contest_type(input: &str) -> Option<ContestType> {
    match input {
        "1" => Some(ContestType::Abc),
        "2" => Some(ContestType::Arc),
        "3" => Some(ContestType::Agc),
        "4" => Some(ContestType::Exit),
        _ => None,
    }
}

fn main() {
    let toml_command = "taplo";
    let cargo_command = "cargo";

    loop {
        println!("Contest type? :");
        for (index, contest_type) in ContestType::iter().enumerate() {
            println!("{}) {:?}", index + 1, contest_type);
        }

        print!("Select number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let choice = input.trim();

        let contest_type = match parse_contest_type(choice) {
            Some(ct) => ct,
            None => {
                println!("Invalid choice: {}", choice);
                continue;
            }
        };

        if let ContestType::Exit = contest_type {
            println!("exit");
            break;
        }

        print!("Contest number?: ");
        io::stdout().flush().unwrap();

        let mut contest_number = String::new();
        io::stdin()
            .read_line(&mut contest_number)
            .expect("Failed to read line");
        let contest_number = contest_number.trim();
        let contest_prefix = match contest_type {
            ContestType::Abc => "ABC",
            ContestType::Arc => "ARC",
            ContestType::Agc => "AGC",
            ContestType::Exit => unreachable!(),
        };
        let project_path = format!("{}/{}", contest_prefix, contest_number);
        if fs::metadata(&project_path).is_ok() {
            eprintln!("Error: Directory '{}' already exists.", project_path);
            exit(1);
        }

        let src_path = format!("{}/src", project_path);
        fs::create_dir_all(&src_path).expect("Failed to create directories");
        let cargo_toml_content = vec![
            "[package]".to_string(),
            format!(
                "name = \"{}_{}\"",
                contest_prefix.to_lowercase(),
                contest_number
            ),
            "version = \"0.1.0\"".to_string(),
            "edition = \"2018\"".to_string(),
            "[dependencies]".to_string(),
            "proconio = \"0.5.0\"".to_string(),
            "itertools = \"0.13.0\"".to_string(),
            "ac-library-rs = \"0.1.1\"".to_string(),
            "superslice = \"1\"\n".to_string(),
        ]
        .iter()
        .join("\n");

        let cargo_toml_path = format!("{}/Cargo.toml", project_path);
        fs::write(&cargo_toml_path, cargo_toml_content)
            .expect("Failed to write project Cargo.toml");

        let problems = ["A", "B", "C", "D", "E", "F", "G", "Ex"];
        for p in problems.iter() {
            let lower = p.to_lowercase();
            let rs_path = format!("{}/{}.rs", src_path, lower);
            let rs_content = "use proconio::input;fn main(){input!{};}";
            fs::write(&rs_path, rs_content).expect("Failed to write .rs file");
            let bin_contents = vec![
                "[[bin]]".to_string(),
                format!("name = \"{}\"", lower),
                format!("path = \"src/{}.rs\"\n", lower),
            ];
            let bin_section = bin_contents.iter().join("\n");
            let mut current_toml =
                fs::read_to_string(&cargo_toml_path).expect("Failed to read project Cargo.toml");
            current_toml.push_str(&bin_section);
            fs::write(&cargo_toml_path, current_toml).expect("Failed to update project Cargo.toml");
        }
        let contest_toml_format_args = vec!["format", &cargo_toml_path];
        run_command(&toml_command, &contest_toml_format_args);

        let workspace_projects = vec![
            format!("\"{}/{}\"", contest_prefix, contest_number),
            "\"ironclad_rule\"".to_string(),
            "\"contest_prepare\"\n".to_string(),
        ]
        .iter()
        .join(",\n");
        let workspace_toml = vec![
            "[workspace]".to_string(),
            format!("members = [{}]", workspace_projects),
            "resolver = \"2\"\n".to_string(),
        ]
        .iter()
        .join("\n");
        fs::write("Cargo.toml", workspace_toml).expect("Failed to write workspace Cargo.toml");
        let root_toml_format_args = vec!["format", "Cargo.toml"];
        run_command(&toml_command, &root_toml_format_args);
        let cargo_fmt_args = vec!["fmt"];
        run_command(&cargo_command, &cargo_fmt_args);
        println!(
            "Project created successfully in {}/{}",
            contest_prefix, contest_number
        );
        break;
    }
}

fn run_command(command: &str, args: &[&str]) {
    let status = Command::new(command).args(args).status();
    match status {
        Ok(exit_status) => {
            println!(
                "Command successfully exited with exit status: {}",
                exit_status
            );
        }
        Err(err) => {
            let error_command = command.to_owned() + " " + &args.iter().join(" ");
            eprintln!(
                "{} returned an error. Error message = {}",
                error_command, err
            );
        }
    }
}
