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

fn write_project_file(
    location_choice: &str,
    file_name: &str,
    content: &str,
) -> std::io::Result<()> {
    let src_exists = Path::new("src").exists();

    let (target_dir, file_path) = match location_choice {
        "Project root" => {
            let dir = if src_exists { "src" } else { "" };
            let path = if src_exists {
                format!("src/{}", file_name)
            } else {
                file_name.to_string()
            };
            (dir, path)
        }
        "lib/" => {
            let dir = if src_exists { "src/lib" } else { "lib" };
            (dir, format!("{}/{}", dir, file_name))
        }
        "utils/" => {
            let dir = if src_exists { "src/utils" } else { "utils" };
            (dir, format!("{}/{}", dir, file_name))
        }
        _ => ("", file_name.to_string()),
    };

    if !target_dir.is_empty() {
        fs::create_dir_all(target_dir)?;
    }
    fs::write(&file_path, content)?;

    println!(
        "{} {}",
        style("✓").bold().green(),
        style(format!("Created {}", file_path)).green()
    );

    Ok(())
}

pub fn write_auth_file(auth_ts_content: &str, location_choice: &str) -> std::io::Result<()> {
    write_project_file(location_choice, "auth.ts", auth_ts_content)
}

pub fn create_api_route(location_choice: &str) -> std::io::Result<()> {
    let src_exists = Path::new("src").exists();

    let auth_import_path = match location_choice {
        "Project root" => "@/auth",
        "lib/" => "@/lib/auth",
        "utils/" => "@/utils/auth",
        _ => "@/auth",
    };

    let route_content = format!(
        "import {{ auth }} from \"{}\";\nimport {{ toNextJsHandler }} from \"better-auth/next-js\";\n\nexport const {{ GET, POST }} = toNextJsHandler(auth.handler);",
        auth_import_path
    );

    let route_dir = if src_exists {
        "src/app/api/auth/[...all]"
    } else {
        "app/api/auth/[...all]"
    };

    fs::create_dir_all(route_dir)?;
    fs::write(format!("{}/route.ts", route_dir), &route_content)?;

    println!(
        "{} {}",
        style("✓").bold().green(),
        style(format!("Created API route in {}", route_dir)).green()
    );

    Ok(())
}

pub fn create_client_file(location_choice: &str) -> std::io::Result<()> {
    let content = "import {\n\
                   \tcreateAuthClient\n\
                   } from \"better-auth/react\";\n\n\
                   export const authClient = createAuthClient({\n\
                   \tbaseURL: process.env.NEXT_PUBLIC_APP_URL,\n\
                   })\n\n\
                   export const {\n\
                   \tsignIn,\n\
                   \tsignOut,\n\
                   \tsignUp,\n\
                   \tuseSession\n\
                   } = authClient;";
    write_project_file(location_choice, "auth-client.ts", content)
}
