use console::style;
use inquire::Select;

// database config
pub struct DatabaseConfig {
    pub config: Option<String>,
    pub packages: Vec<String>,
    pub imports: Option<String>,
    pub pre_config: Option<String>,
}

pub fn setup_database_config() -> DatabaseConfig {
    println!("\n{}", style("Database Configuration").bold().blue());

    // database options
    let database_options = vec!["None", "MySQL", "PostgreSQL", "SQLite", "MS SQL"];

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
    let mut pre_config = None;

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
                "\tdatabase: new Pool({\n\t\tconnectionString: \"postgres://user:password@localhost:5432/database\",\n\t})",
            ));
        }
        "SQLite" => {
            // add better-sqlite3 package
            packages.push(String::from("better-sqlite3"));
            // add better-sqlite3 imports
            imports = Some(String::from("import Database from \"better-sqlite3\";"));
            // add better-sqlite3 config
            config = Some(String::from(
                "\tdatabase: new Database(\"database.sqlite\")",
            ));
        }
        "MS SQL" => {
            // add kysely and tedious packages
            packages.push(String::from("kysely"));
            packages.push(String::from("tedious"));
            packages.push(String::from("tarn"));
            // add imports
            imports = Some(String::from(
                "import { MssqlDialect } from \"kysely\";\nimport * as Tedious from 'tedious';\nimport * as Tarn from 'tarn';",
            ));
            // add MS SQL pre-config (dialect definition)
            pre_config = Some(String::from(
                "const dialect = new MssqlDialect({\n\ttarn: {\n\t\t...Tarn,\n\t\toptions: {\n\t\t\tmin: 0,\n\t\t\tmax: 10,\n\t\t},\n\t},\n\ttedious: {\n\t\t...Tedious,\n\t\tconnectionFactory: () => new Tedious.Connection({\n\t\t\tauthentication: {\n\t\t\t\toptions: {\n\t\t\t\t\tpassword: 'password',\n\t\t\t\t\tuserName: 'username',\n\t\t\t\t},\n\t\t\t\ttype: 'default',\n\t\t\t},\n\t\t\toptions: {\n\t\t\t\tdatabase: 'some_db',\n\t\t\t\tport: 1433,\n\t\t\t\ttrustServerCertificate: true,\n\t\t\t},\n\t\t\tserver: 'localhost',\n\t\t}),\n\t},\n});",
            ));
            // add MS SQL config
            config = Some(String::from(
                "\tdatabase: {\n\t\tdialect,\n\t\ttype: \"mssql\"\n\t}",
            ));
        }
        _ => (),
    }

    // return database config
    DatabaseConfig {
        config,
        packages,
        imports,
        pre_config,
    }
}
