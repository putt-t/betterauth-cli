use console::{Term, style};
use inquire::Select;
use std::env;
use std::fs;
use std::process::Command;

// imports
use super::auth_file::{generate_auth_ts_content, write_auth_file};
use super::database::setup_database_config;
use super::social::{setup_social_providers, write_env_vars};

fn print_step_header(step_number: usize, total_steps: usize, title: &str) {
    // clear line
    let term = Term::stdout();
    let _ = term.clear_line();
    // print step header
    println!(
        "\n{} {} {}\n{}",
        style(format!("[{}/{}]", step_number, total_steps))
            .bold()
            .cyan(),
        style("•").bold().yellow(),
        style(title).bold().white(),
        style("━".repeat(60)).dim()
    );
}

fn print_success(message: &str) {
    println!("{} {}", style("✓").bold().green(), style(message).green());
}

fn print_info(message: &str) {
    println!("{} {}", style("ℹ").bold().blue(), style(message).dim());
}

pub fn setup_nextjs_project(project_name: String, better_auth_secret: String) {
    let total_steps = 5;
    let term = Term::stdout();

    // clear the terminal for a fresh start
    let _ = term.clear_screen();

    // print header
    println!(
        "{}",
        style("BetterAuth CLI - Next.js Setup")
            .bold()
            .magenta()
            .underlined()
    );
    println!(
        "{}\n",
        style("Creating your secure authentication system")
            .italic()
            .dim()
    );

    // step 1: initialize next.js project
    print_step_header(1, total_steps, "Initializing Next.js project");
    print_info(&format!("Creating new Next.js project: {}", project_name));

    // create next.js project
    let npx_result = Command::new("npx")
        .arg("create-next-app@latest")
        .arg(&project_name)
        .status();

    // match result
    match npx_result {
        Ok(_) => print_success("Next.js project created successfully"),
        Err(e) => {
            println!(
                "{} {}",
                style("ERROR:").bold().red(),
                style(format!("Failed to create Next.js project: {}", e)).red()
            );
            return;
        }
    }

    // step 2: setup project
    print_step_header(2, total_steps, "Setting up project");
    print_info(&format!("Changing directory to {}", project_name));

    if let Err(e) = env::set_current_dir(&project_name) {
        println!(
            "{} {}",
            style("ERROR:").bold().red(),
            style(format!("Failed to change directory: {}", e)).red()
        );
        return;
    }

    // install better-auth package
    print_info("Installing better-auth package");
    let npm_result = Command::new("npm")
        .arg("install")
        .arg("better-auth")
        .status();

    // match result
    match npm_result {
        Ok(_) => print_success("better-auth installed successfully"),
        Err(e) => {
            println!(
                "{} {}",
                style("ERROR:").bold().red(),
                style(format!("Failed to install better-auth: {}", e)).red()
            );
            return;
        }
    }

    // step 3: create environment file
    print_step_header(3, total_steps, "Creating environment configuration");
    print_info("Creating .env.local file");

    // write to .env.local
    match fs::write(
        ".env.local",
        format!(
            "BETTER_AUTH_SECRET={}\nBETTER_AUTH_URL=http://localhost:3000 #Base URL of your app",
            better_auth_secret
        ),
    ) {
        Ok(_) => print_success(".env.local created with authentication secret"),
        Err(e) => {
            println!(
                "{} {}",
                style("ERROR:").bold().red(),
                style(format!("Failed to create .env.local: {}", e)).red()
            );
            return;
        }
    }

    // step 4: configure auth location
    print_step_header(4, total_steps, "Configuring auth location");

    // prompt for auth location
    let create_better_auth_instance = Select::new(
        &style("Where do you want to create better auth instance?")
            .bold()
            .to_string(),
        vec!["Project root", "lib/", "utils/"],
    )
    .with_help_message(
        &style("Select the location for your auth.ts file")
            .dim()
            .to_string(),
    )
    .prompt()
    .unwrap_or("Project root");

    // step 5: configure authentication
    print_step_header(5, total_steps, "Configuring authentication");

    // get database and social provider configurations
    let database_config = setup_database_config();
    let social_providers_config = setup_social_providers();

    // generate auth.ts content based on configurations
    let auth_ts_content = generate_auth_ts_content(&database_config, &social_providers_config);

    // write environment variables to .env.local
    write_env_vars(&social_providers_config.env_vars, ".env.local");

    // install additional packages if needed
    if !database_config.packages.is_empty() {
        print_info(&format!(
            "Installing additional packages: {}",
            database_config.packages.join(", ")
        ));

        let npm_install_result = Command::new("npm")
            .arg("install")
            .args(&database_config.packages)
            .status();

        // match result
        match npm_install_result {
            Ok(_) => print_success("Additional packages installed successfully"),
            Err(e) => {
                println!(
                    "{} {}",
                    style("ERROR:").bold().red(),
                    style(format!("Failed to install packages: {}", e)).red()
                );
            }
        }
    }

    // write auth.ts file to the selected location
    write_auth_file(&auth_ts_content, &create_better_auth_instance);

    // final success message
    let _ = term.clear_line();
    println!("\n{}", style("━".repeat(60)).dim());
    println!(
        "{} {}",
        style("SETUP COMPLETE!").bold().green(),
        style("Your Next.js project with Better Auth is almost ready!").green()
    );
    println!("{}", style("━".repeat(60)).dim());

    // next steps
    println!("\n{}", style("Next Steps:").bold().yellow());
    println!(
        "  {} {}",
        style("•").yellow(),
        style("Add in environment variables to .env.local").white()
    );
    println!(
        "\n{}",
        style("Documentation: https://better-auth.com/docs")
            .italic()
            .dim()
    );
}
