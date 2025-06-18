use inquire::{Select};
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

    // match the user's choice
    match create_better_auth_instance {
        "Project root" => {
            fs::write(
                "auth.ts",
                "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});",
            )
            .unwrap();
        }
        "lib/" => {
            // first check if src exists
            if Path::new("src").exists() {
                // make a new directory called lib
                std::fs::create_dir("src/lib").unwrap();
                std::fs::write("src/lib/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
            } else {
                // make a new directory called lib
                fs::create_dir("lib").unwrap();
                fs::write("lib/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
            }
        }
        "utils/" => {
            // first check if src exists
            if Path::new("src").exists() {
                // make a new directory called utils
                fs::create_dir("src/utils").unwrap();
                fs::write("src/utils/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
            } else {
                // make a new directory called utils
                fs::create_dir("utils").unwrap();
                fs::write("utils/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
            }
        }
        _ => println!("Invalid choice"),
    };
}
