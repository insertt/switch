use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "switch", author = "", about = "Switch between environment variables in current session easily.")]
pub enum SwitchCommand {
    #[structopt(name = "add")]
    /**
     * Add environment variable to given category.
     * Note: It will create category if there isn't declared one with given name.
     */
    Add {
        #[structopt(name = "category", help = "Category of declared environment variable")]
        category: String,
        #[structopt(name = "name", help = "Name of declared environment variable")]
        name: String,
        #[structopt(name = "value", help = "Value of declared environment variable")]
        value: String
    },
    #[structopt(name = "remove")]
    /// Remove declared environment variable from category.
    Remove {
        #[structopt(name = "category", help = "Category of environment variable for removal.")]
        category: String,
        #[structopt(name = "name", help = "Name of declared environment variable for removal.")]
        name: Option<String>
    },
    #[structopt(name = "apply")]
    /// Apply declared environment variable.
    Apply {
        #[structopt(name = "category", help = "Category of declared environment variable", default_value = "")]
        category: String,
        #[structopt(name = "name", help = "Name of declared environment variable", default_value = "")]
        name: String
    },
    #[structopt(name = "list")]
    /// List declared categories and declared environment variables.
    List
}