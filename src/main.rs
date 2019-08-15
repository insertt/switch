#[macro_use]
extern crate serde_derive;

mod switch;
mod command;
mod opt;

use std::error::{ Error };
use switch::{ SwitchRegistry };
use structopt::{ StructOpt };
use opt::{ SwitchCommand, SwitchCommand::* };

const REGISTRY_PATH: &str = ".config/switch/registry.json";

fn main() -> Result<(), Box<dyn Error>> { 
    let registry_dir = match dirs::home_dir() {
        Some(mut dir) => {
            dir.push(REGISTRY_PATH);
            dir
        },
        _ => panic!()
    };
    
    let mut registry = SwitchRegistry::deserialize_from_file(&registry_dir)?;

    match SwitchCommand::from_args() {
        Add { category, name, value } => {
            command::add(&mut registry, category, name, value);
            registry.serialize_to_file(&registry_dir)?;
        },
        Remove { category, name } => {
            command::remove(&mut registry, category, name);
            registry.serialize_to_file(&registry_dir)?;
        },
        Apply { category, name } => {
            command::apply(&mut registry, category, name);
        },
        List => {
            command::list(registry);
        }
    }

    Ok(())
}
