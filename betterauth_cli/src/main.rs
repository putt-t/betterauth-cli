mod frameworks;
mod utils;

use inquire::{Select, Text};

use frameworks::Framework;
use frameworks::nextjs;
use utils::generate_better_auth_secret;

fn main() {
    let better_auth_secret = generate_better_auth_secret();

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
            nextjs::setup_nextjs_project(project_name, better_auth_secret);
        }
        _ => println!("Not implemented"),
    }
}
