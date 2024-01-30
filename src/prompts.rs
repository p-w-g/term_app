use std::{
    fs::File,
    io::{self, Write},
};

pub fn get_home() -> String {
    let mut home_dir: String = String::new();

    if let Ok(value) = std::env::var("HOME") {
        home_dir = value
    } else {
        println!("Environment variable not set");
    }

    home_dir
}

fn get_input(msg: &str) -> String {
    println!("Type in your {msg}");
    let mut users_input = String::new();

    io::stdin()
        .read_line(&mut users_input)
        .expect("Failed to read the line");

    let users_input: String = users_input
        .trim()
        .parse()
        .expect("You were supposed to type in {msg}");

    users_input
}

pub fn write_git_conf() {
    let home = get_home();
    let gitconfig_path = format!("{}/.gitconfig", home);

    if let Ok(_) = File::open(&gitconfig_path) {
        println!("The .gitconfig file already exists.");
    } else {
        let full_name = get_input("full name");
        let email = get_input("email");
        match File::create(&gitconfig_path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();

                let _template = write!(
                    buffer,
                    "[user]\n  name = {}\n  email = {}\n",
                    full_name, email
                );

                file.write_all(&buffer).unwrap();

                println!("The .gitconfig file has been created.");
            }
            Err(err) => {
                eprintln!("Error creating .gitconfig file: {}", err);
                std::process::exit(1);
            }
        }
    }
}

pub fn write_npmrc() {
    let home = get_home();
    let npmrc = format!("{}/.npmrc", home);

    if let Ok(_) = File::open(&npmrc) {
        println!("The .npmrc file already exists.");
    } else {
        let pat = get_input("PAT");
        match File::create(&npmrc) {
            Ok(mut file) => {
                let mut buffer = Vec::new();

                let _template = write!(
                    buffer,
                    "first_line\nsecond_line\nthird_line_with_var={}\n",
                    pat
                );

                file.write_all(&buffer).unwrap();

                println!("The .npmrc file has been created.");
            }
            Err(err) => {
                eprintln!("Error creating .npmrc file: {}", err);
                std::process::exit(1);
            }
        }
    }
}
