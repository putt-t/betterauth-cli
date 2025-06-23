use console::style;
use inquire::MultiSelect;
use std::fs;
use std::io::Write;

// social prov config
pub struct SocialProvidersConfig {
    pub config: Option<String>,
    pub env_vars: String,
    pub providers: Vec<String>,
}

pub fn setup_social_providers() -> SocialProvidersConfig {
    println!("\n{}", style("Social Login Configuration").bold().magenta());

    // options for social providers
    let social_providers = vec![
        "Google", "GitHub", "Discord", "Facebook", "Twitter", "Apple", "HuggingFace", "Kick", "Microsoft", "Tiktok", "Twitch", "Twitter", "Dropbox", "Linkedin", "Gitlab", "Reddit", "Roblox", "Spotify", "Vk", "Zoom"
    ];

    // prompt for social providers
    println!(
        "{} {}",
        style("ℹ").bold().blue(),
        style("Press SPACE to select providers, then ENTER to confirm").dim()
    );

    // select social providers
    let selected_providers = MultiSelect::new(
        &style("Which social login providers would you like to use?")
            .bold()
            .to_string(),
        social_providers,
    )
    .with_help_message(
        &style("↑↓ to move, SPACE to select, ENTER to confirm")
            .dim()
            .to_string(),
    )
    .prompt()
    .unwrap_or_default();

    // if no social providers selected, return empty config
    if selected_providers.is_empty() {
        println!(
            "{} {}",
            style("ℹ").bold().blue(),
            style("No social providers selected. You can add them manually later.").dim()
        );
        return SocialProvidersConfig {
            config: None,
            env_vars: String::new(),
            providers: Vec::new(),
        };
    }

    // create a string with provider names for display
    let providers_display = selected_providers.join(", ");
    println!(
        "{} {}",
        style("Selected:").dim(),
        style(&providers_display).bold().green()
    );

    // create social config
    let mut social_config = String::from("\tsocialProviders: {\n");
    let mut all_env_vars = String::new();

    // add providers to social config
    for provider in &selected_providers {
        let provider_lowercase = provider.to_lowercase();

        // add environment variables
        all_env_vars.push_str(&format!(
            "\n{0}_CLIENT_ID=your_{1}_client_id\n{0}_CLIENT_SECRET=your_{1}_client_secret",
            provider.to_uppercase(),
            provider_lowercase
        ));

        // add provider to social_config
        if *provider == "Tiktok" {
            social_config.push_str(&format!(
                "\t\t{}: {{ \n\t\t\tclientId: process.env.{}_CLIENT_ID as string,\n\t\t\tclientSecret: process.env.{}_CLIENT_SECRET as string,\n\t\t\tclientKey: process.env.{}_CLIENT_KEY as string,\n\t\t}},\n",
                provider_lowercase,
                provider.to_uppercase(),    
                provider.to_uppercase(),
                provider.to_uppercase()
            ));
        } else if *provider == "Gitlab" {
            social_config.push_str(&format!(
                "\t\t{}: {{ \n\t\t\tclientId: process.env.{}_CLIENT_ID as string,\n\t\t\tclientSecret: process.env.{}_CLIENT_SECRET as string,\n\t\t\tissuer: process.env.{}_ISSUER as string,\n\t\t}},\n",
                provider_lowercase,
                provider.to_uppercase(),    
                provider.to_uppercase(),
                provider.to_uppercase()
            ));
        } else {
            social_config.push_str(&format!(
                "\t\t{}: {{ \n\t\t\tclientId: process.env.{}_CLIENT_ID as string,\n\t\t\tclientSecret: process.env.{}_CLIENT_SECRET as string,\n\t\t}},\n",
                provider_lowercase,
                provider.to_uppercase(),    
                provider.to_uppercase()
            ));
        }


    }

    // push dat shiii
    social_config.push_str("\t}");

    // return social config
    SocialProvidersConfig {
        config: Some(social_config),
        env_vars: all_env_vars,
        providers: selected_providers.iter().map(|s| s.to_string()).collect(),
    }
}

// write env vars to .env.local
pub fn write_env_vars(env_vars: &str, env_file_path: &str) {
    if !env_vars.is_empty() {
        // open .env.local file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(env_file_path)
            .unwrap();
        // write env vars to .env.local
        file.write_all(env_vars.as_bytes()).unwrap();
        println!(
            "{} {}",
            style("✓").bold().green(),
            style(format!("Added environment variables to {}", env_file_path)).green()
        );
    } else {
        println!(
            "{} {}",
            style("✓").bold().green(),
            style(format!(
                "No environment variables to add to {}",
                env_file_path
            ))
            .green()
        );
    }
}
