use inquire::{InquireError, Select, Text};

fn main() {
    let mut fruits: Vec<String> = vec!["Banana".into(), "Apple".into()];

    let menu_options: Vec<&str> = vec!["Show fruits", "Add a fruit", "Remove a fruit", "Exit"];
    loop {
        let menu_ans: Result<&str, InquireError> =
            Select::new("What would you like to do?", menu_options.clone())
                .with_help_message("Use HJKL to navigate, and Enter to select")
                .with_vim_mode(true)
                .prompt();

        match menu_ans {
            Ok(choice) => match choice {
                "Show fruits" => show_fruits(&fruits),
                "Add a fruit" => add_fruit(&mut fruits),
                "Remove a fruit" => remove_fruit(&mut fruits),
                "Exit" => break,
                _ => println!("Invalid choice"),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
}

fn show_fruits(fruits: &Vec<String>) {
    println!("Fruits:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn add_fruit(fruits: &mut Vec<String>) {
    let fruit_name: Result<String, InquireError> =
        Text::new("Enter the name of the fruit to add:").prompt();

    match fruit_name {
        Ok(name) => {
            fruits.push(name);
            println!("Fruit added.");
        }
        Err(_) => println!("Failed to get the fruit name."),
    }
}

fn remove_fruit(fruits: &mut Vec<String>) {
    if fruits.is_empty() {
        println!("No fruits to remove.");
        return;
    }

    let remove_ans: Result<String, InquireError> =
        Select::new("Select a fruit to remove:", fruits.clone())
            .with_help_message("Use HJKL to navigate, and Enter to select")
            .with_vim_mode(true)
            .prompt();

    match remove_ans {
        Ok(fruit) => {
            fruits.retain(|f| f != &fruit);
            println!("Fruit removed.");
        }
        Err(_) => println!("Failed to select a fruit to remove."),
    }
}
