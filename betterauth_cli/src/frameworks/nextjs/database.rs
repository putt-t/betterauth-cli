use console::style;
use inquire::Select;

// database config
pub struct DatabaseConfig {
    pub config: Option<String>,
    pub packages: Vec<String>,
    pub imports: Option<String>,
}

pub fn setup_database_config() -> DatabaseConfig {
    println!("\n{}", style("Database Configuration").bold().blue());

    // database options
    let database_options = vec!["None", "MySQL", "PostgreSQL", "SQLite", "MongoDB"];

    // prompt for database
    let database_choice = Select::new(
        &style("Which database would you like to use?")
            .bold()
            .to_string(),
        database_options,
    )
    .with_help_message(
        &style("Your database will store user information")
            .dim()
            .to_string(),
    )
    .prompt()
    .unwrap_or("None");

    // if database selected, print it
    if database_choice != "None" {
        println!(
            "{} {}",
            style("Selected:").dim(),
            style(database_choice).bold().green()
        );
    }

    // initialize packages, config, and imports
    let mut packages = Vec::new();
    let mut config = None;
    let mut imports = None;

    // match database choice
    match database_choice {
        "None" => (),
        "MySQL" => {
            // add mysql2 package
            packages.push(String::from("mysql2"));
            imports = Some(String::from(
                "import { createPool } from \"mysql2/promise\";",
            ));
            // add mysql2 config
            config = Some(String::from(
                "\tdatabase: createPool({\n\t\thost: \"localhost\",\n\t\tuser: \"root\",\n\t\tpassword: \"password\",\n\t\tdatabase: \"database\",\n\t})",
            ));
        }
        "PostgreSQL" => {
            // add pg package
            packages.push(String::from("pg"));
            // add pg imports
            imports = Some(String::from("import { Pool } from \"pg\";"));
            // add pg config
            config = Some(String::from(
                "\tdatabase: new Pool({\n\t\thost: \"localhost\",\n\t\tuser: \"postgres\",\n\t\tpassword: \"password\",\n\t\tdatabase: \"database\",\n\t\tport: 5432,\n\t})",
            ));
        }
        "SQLite" => {
            // add better-sqlite3 package
            packages.push(String::from("better-sqlite3"));
            // add better-sqlite3 imports
            imports = Some(String::from("import Database from \"better-sqlite3\";"));
            // add better-sqlite3 config
            config = Some(String::from("\tdatabase: new Database(\"./database.db\")"));
        }
        "MongoDB" => {
            // add mongodb package
            packages.push(String::from("mongodb"));
            // add mongodb imports
            imports = Some(String::from("import { MongoClient } from \"mongodb\";"));
            // add mongodb config
            config = Some(String::from(
                "\tdatabase: new MongoClient(\"mongodb://localhost:27017\").db(\"database\")",
            ));
        }
        _ => (),
    }

    // return database config
    DatabaseConfig {
        config,
        packages,
        imports,
    }
}
