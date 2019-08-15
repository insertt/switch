use std::io::{ stdin };
use std::fs;
use std::process::{ exit };
use crate::switch::{ SwitchRegistry, SwitchCategory };
use ttyecho::{ ttyecho };

pub fn set(registry: &mut SwitchRegistry, category_name: String, name: String, value: String) {
    let category = registry.get_category(&category_name);

    match category {
        Some(category) => {
            if ! category.add_variable(&name, &value) {
                eprintln!("There is already registered environment variable with key '{}' in category '{}'!", &name, &category_name);
                return;
            }
        }
        None => {
            let mut category = SwitchCategory::new(&category_name);
            category.add_variable(&name, &value);
            registry.add_category(category);
        }
    }

    println!("Added entry '{}' -> '{}' to category '{}'", name, value, category_name);
}

pub fn remove(registry: &mut SwitchRegistry, category_name: String, name: Option<String>) {
    let category = registry.get_category(&category_name);

    match category {
        Some(category) => {
            if let Some(name) = name {
                if category.remove_variable(&name) {
                    println!("Removed key '{}' from category '{}'", name, category_name);
                }
                else {
                    println!("Given key '{}' does not have mapped value in category '{}'", name, category_name);
                }
            }
            else {
                registry.remove_category(&category_name);
                println!("Category '{}' has been removed!", category_name);
            }
        }
        None => {
            eprintln!("Category '{}' does not exists!", category_name);
        }
    }
}

pub fn apply(registry: &mut SwitchRegistry, mut category_name: String , mut name: String) {
    if category_name.is_empty() {
        println!("{}", registry);
        println!("Please type in category of variable to apply: ");
        stdin().read_line(&mut category_name).unwrap_or_else(|_| {
            eprintln!("Sorry, but this is not a valid category name.");
            exit(1);
        });

        category_name = category_name.trim().to_string();
    }

    if name.is_empty() {
        println!("Please type in name of the variable to apply: ");
        stdin().read_line(&mut name).unwrap_or_else(|_| {
            eprintln!("Sorry, but this is not a valid variable name.");
            exit(1);
        });

        name = name.trim().to_string();
    }

    let category = registry.get_category(&category_name);

    match category {
        Some(category) => {
            let pts = match fs::canonicalize("/proc/self/fd/0") {
                Ok(pts) => pts,
                Err(why) => panic!("Could not retrieve pty: {:#?}", why)
            };

            match category.get_variable(&name) {
                Some(env) => {
                    ttyecho(pts.to_str().unwrap().to_string(), format!("export {} && clear", env.value), true);
                    println!("Environment variables from category '{}' applied!", category_name);
                }
                _ => {
                    eprintln!("Could not find environment variable with name '{}' in category '{}'", &name, &category_name);
                }
            };   
        },
        None => {
            eprintln!("Given category name is not registered!");
        }
    }
}

pub fn list(registry: SwitchRegistry) {
    print!("{}", registry);
}
