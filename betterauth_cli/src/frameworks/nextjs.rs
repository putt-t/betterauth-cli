use inquire::Select;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn setup_nextjs_project(project_name: String, better_auth_secret: String) {
    println!("Initializing Next.js project...");
    // create a new next.js project
    Command::new("npx")
        .arg("create-next-app@latest")
        .arg(&project_name)
        .status()
        .unwrap();
    // cd into the project
    println!("cd into the project...");
    env::set_current_dir(project_name).unwrap();
    println!("installing better-auth...");
    // npm install better-auth
    Command::new("npm")
        .arg("install")
        .arg("better-auth")
        .status()
        .unwrap();
    // create a new .env file
    println!("creating .env.local...");
    fs::write(
        ".env.local",
        format!(
            "BETTER_AUTH_SECRET={}\nBETTER_AUTH_URL=http://localhost:3000 #Base URL of your app",
            better_auth_secret
        ),
    )
    .unwrap();

    // let user choose where they want to create better auth instance
    let create_better_auth_instance = Select::new(
        "Where do you want to create better auth instance? (auth.ts)",
        vec!["Project root", "lib/", "utils/"],
    )
    .prompt()
    .unwrap();

    let auth_ts_content = fs::read_to_string("../../templates/nextjs/auth.ts.template")
        .expect("Failed to read auth.ts template file");

    // match the user's choice
    match create_better_auth_instance {
        "Project root" => {
            fs::write("auth.ts", &auth_ts_content).unwrap();
        }
        "lib/" => {
            // first check if src exists
            let target_dir = if Path::new("src").exists() {
                "src/lib"
            } else {
                "lib"
            };
            fs::create_dir_all(target_dir).unwrap();
            fs::write(format!("{}/auth.ts", target_dir), &auth_ts_content).unwrap();
        }
        "utils/" => {
            // first check if
            let target_dir = if Path::new("src").exists() {
                "src/utils"
            } else {
                "utils"
            };
            fs::create_dir_all(target_dir).unwrap();
            fs::write(format!("{}/auth.ts", target_dir), &auth_ts_content).unwrap();
        }
        _ => println!("Invalid choice"),
    };
    // ask user if they want to continue on with database setup
    let continue_with_database_setup = Select::new(
        "Do you want to continue on with database setup?",
        vec!["Yes", "No"],
    )
    .prompt()
    .unwrap();

    // match the user's choice
    match continue_with_database_setup {
        "Yes" => {
            println!("Continuing on with database setup...");
        }
        "No" => {
            println!("Skipping database setup...");
        }
        _ => println!("Invalid choice"),
    }
}
