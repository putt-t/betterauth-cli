use base64::Engine;
use inquire::{Select, Text};
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
enum Framework {
    Astro,
    Remix,
    Next,
    Nuxt,
    SvelteKit,
    SolidStart,
    TanstackStart,
}

impl std::fmt::Display for Framework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Framework::Astro => write!(f, "Astro"),
            Framework::Remix => write!(f, "Remix"),
            Framework::Next => write!(f, "Next"),
            Framework::Nuxt => write!(f, "Nuxt"),
            Framework::SvelteKit => write!(f, "SvelteKit"),
            Framework::SolidStart => write!(f, "SolidStart"),
            Framework::TanstackStart => write!(f, "Tanstack Start"),
        }
    }
}

fn main() {
    // generate byte array of 32 bytes
    let mut rng = rand::rng();
    let mut random_data: [u8; 32] = [0; 32];
    // fill the array with random data
    rng.fill(&mut random_data);
    // encode the array to base64
    let random_data_base64 = base64::engine::general_purpose::STANDARD.encode(random_data);

    // options for the framework selection
    let options: Vec<Framework> = vec![
        Framework::Astro,
        Framework::Remix,
        Framework::Next,
        Framework::Nuxt,
        Framework::SvelteKit,
        Framework::SolidStart,
        Framework::TanstackStart,
    ];
    // prompt the user to select a framework
    let selected_option = Select::new("Choose your framework", options)
        .prompt()
        .unwrap();
    // print the selected option
    println!("You selected: {}", selected_option);

    // ask for the project name
    // loop until the project name is not empty
    let mut project_name = String::new();
    while project_name.is_empty() {
        project_name = Text::new("Enter the project name").prompt().unwrap();
    }

    // match the selected option
    match selected_option {
        // Next.js
        Framework::Next => {
            println!("Initializing Next.js project...");
            // create a new next.js project
            std::process::Command::new("npx")
                .arg("create-next-app@latest")
                .arg(&project_name)
                .status()
                .unwrap();
            // cd into the project
            println!("cd into the project...");
            std::env::set_current_dir(project_name).unwrap();
            println!("installing better-auth...");
            // npm install better-auth
            std::process::Command::new("npm")
                .arg("install")
                .arg("better-auth")
                .status()
                .unwrap();
            // create a new .env file
            println!("creating .env.local...");
            std::fs::write(
                ".env.local",
                format!("BETTER_AUTH_SECRET={}\nBETTER_AUTH_URL=http://localhost:3000 #Base URL of your app", random_data_base64),
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
                    std::fs::write(
                        "auth.ts",
                        "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});",
                    )
                    .unwrap();
                }
                "lib/" => {
                    // first check if src exists
                    if std::path::Path::new("src").exists() {
                        // make a new directory called lib
                        std::fs::create_dir("src/lib").unwrap();
                        std::fs::write("src/lib/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
                    } else {
                        // make a new directory called lib
                        std::fs::create_dir("lib").unwrap();
                        std::fs::write("lib/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
                    }
                }
                "utils/" => {
                    // first check if src exists
                    if std::path::Path::new("src").exists() {
                        // make a new directory called utils
                        std::fs::create_dir("src/utils").unwrap();
                        std::fs::write("src/utils/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
                    } else {
                        // make a new directory called utils
                        std::fs::create_dir("utils").unwrap();
                        std::fs::write("utils/auth.ts", "import { betterAuth } from \"better-auth\";\n\nexport const auth = betterAuth({\n\t//...\n});").unwrap();
                    }
                }
                _ => println!("Invalid choice"),
            };
        }
        _ => println!("Not implemented"),
    }
}
