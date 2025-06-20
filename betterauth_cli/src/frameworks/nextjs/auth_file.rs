use console::style;
use std::fs;
use std::path::Path;

use super::database::DatabaseConfig;
use super::social::SocialProvidersConfig;

pub fn generate_auth_ts_content(
    database_config: &DatabaseConfig,
    social_providers_config: &SocialProvidersConfig,
) -> String {
    let mut imports = String::from("import { betterAuth } from \"better-auth\";\n");
    let mut config_parts = Vec::new();
    let mut pre_config = String::new();

    // add database import if needed
    if let Some(import_str) = &database_config.imports {
        imports.push_str(import_str);
        imports.push('\n');
    }

    // add pre-config if needed (for MS SQL dialect)
    if let Some(pre_config_str) = &database_config.pre_config {
        pre_config.push_str(pre_config_str);
        pre_config.push('\n');
    }

    // add configurations
    if let Some(social_config) = &social_providers_config.config {
        config_parts.push(social_config);
    }

    // add database config if needed
    if let Some(db_config) = &database_config.config {
        config_parts.push(db_config);
    }

    // make that auth.ts file brah
    let config = if config_parts.is_empty() {
        String::from("\t//...")
    } else {
        // join the string references
        let parts: Vec<&str> = config_parts.iter().map(|s| s.as_str()).collect();
        parts.join(",\n")
    };

    // format the auth.ts file
    format!(
        "{}\n{}\nexport const auth = betterAuth({{\n{}\n}});",
        imports, pre_config, config
    )
}

pub fn write_auth_file(auth_ts_content: &str, location_choice: &str) {
    match location_choice {
        // if the user wants to put it in the project root
        "Project root" => {
            // create the file if it doesn't exist
            fs::write("auth.ts", auth_ts_content).unwrap();
            println!(
                "{} {}",
                style("✓").bold().green(),
                style("Created auth.ts in project root").green()
            );
        }
        // if the user wants to put it in the lib folder
        "lib/" => {
            // first check if src exists
            let target_dir = if Path::new("src").exists() {
                "src/lib"
            } else {
                "lib"
            };
            // create the directory if it doesn't exist
            fs::create_dir_all(target_dir).unwrap();
            fs::write(format!("{}/auth.ts", target_dir), auth_ts_content).unwrap();
            println!(
                "{} {}",
                style("✓").bold().green(),
                style(format!("Created auth.ts in {}", target_dir)).green()
            );
        }
        // if the user wants to put it in the utils folder
        "utils/" => {
            // first check if src exists
            let target_dir = if Path::new("src").exists() {
                "src/utils"
            } else {
                "utils"
            };
            // create the directory if it doesn't exist
            fs::create_dir_all(target_dir).unwrap();
            fs::write(format!("{}/auth.ts", target_dir), auth_ts_content).unwrap();
            println!(
                "{} {}",
                style("✓").bold().green(),
                style(format!("Created auth.ts in {}", target_dir)).green()
            );
        }
        _ => println!(
            "{} {}",
            style("!").bold().yellow(),
            style("Invalid choice for auth.ts location").yellow()
        ),
    }
}
