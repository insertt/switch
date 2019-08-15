use std::fs;
use std::fs::{ File, OpenOptions };
use std::error::{ Error };
use std::io::{ Write, BufReader, BufWriter };
use std::path::{ Path, PathBuf };
use std::fmt::{ Display, Formatter };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchRegistry {
    pub categories: Vec<SwitchCategory>
}

#[allow(unused_must_use)]
impl Display for SwitchRegistry {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        for (idx, category) in self.categories.iter().enumerate() {
            write!(formatter, "{}", category);

            if idx != self.categories.len() - 1 {
                writeln!(formatter);
            }
        }

        Ok(())
    }
}

impl SwitchRegistry {
    pub fn new() -> Self {
        SwitchRegistry {
            categories: Vec::new()
        }
    }

    pub fn get_category<S: Into<String>>(&mut self, category_name: S) -> Option<&mut SwitchCategory> {
        let category_name = category_name.into();
        for category in self.categories.iter_mut() {
            if category.name == category_name {
                return Some(category);
            }
        }
        
        None
    }

    pub fn add_category(&mut self, category: SwitchCategory) {
        self.categories.push(category);
    }

    pub fn remove_category<S: Into<String>>(&mut self, category_name: S) -> bool {
        let category_name = category_name.into();
        for (idx, category) in self.categories.iter_mut().enumerate() {
            if category.name == category_name {
                self.categories.remove(idx);
                return true;
            }
        }

        false
    }

    pub fn serialize_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new().write(true).truncate(true).open(path.as_ref())?;
        let mut writer = BufWriter::new(file);

        let serialized_registry = serde_json::to_string_pretty(self)?;

        if let Err(why) = writer.write_all(serialized_registry.as_bytes()) {
            eprintln!("Could not write serialized registry into buffer.");
            eprintln!("Reason: {:#?}", why);
        }

        if let Err(why) = writer.flush() {
            eprintln!("Could not flush writer.");
            eprintln!("Reason: {:#?}", why);
        }

        Ok(())
    }

    pub fn deserialize_from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(&path).unwrap_or_else(|_| {
            if let Err(why) = fs::create_dir_all(&path.parent().unwrap()) {
                panic!("Could not create default registy directory: {:#?}", why);
            }

            match File::create(path) {
                Ok(file) => file,
                Err(why) => panic!("Could not create default registry file: {:#?}", why)
            }
        });

        if file.metadata().expect("Could not retrieve file metadata").len() == 0 {
            return Ok(SwitchRegistry::new());
        }

        let reader = BufReader::new(file);

        let registry = serde_json::from_reader(reader).expect("Could not read from registry file");
        Ok(registry)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchCategory {
    pub name: String,
    pub variables: Vec<SwitchVariable>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchVariable {
    pub key: String,
    pub value: String
}

impl SwitchVariable {
    fn new<S: Into<String>>(key: S, value: S) -> Self {
        SwitchVariable {
            key: key.into(),
            value: value.into()
        }
    }
}

#[allow(unused_must_use)]
impl Display for SwitchCategory {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        let header = format!("Category: {}", self.name);

        writeln!(formatter, "┌─ {}", header);
        writeln!(formatter, "| ");

        for (idx, variable) in self.variables.iter().enumerate() {
            let padding: usize;
            
            if idx == self.variables.len() - 1 {
                writeln!(formatter, "└─ {}", variable.key);
                padding = 2;
            }
            else {
                writeln!(formatter, "├─ {}", variable.key);
                write!(formatter, "| ");
                padding = 0
            }
            
            for _ in 0..variable.key.len() + padding {
                write!(formatter, " ");
            }

            writeln!(formatter, "└─ {}", variable.value);
        }

        Ok(())
    }
}

impl SwitchCategory {
    pub fn new<S: Into<String>>(name: S) -> Self {
        SwitchCategory {
            name: name.into(),
            variables: Vec::new()
        }
    }

    pub fn add_variable<S: Into<String>>(&mut self, key: S, value: S) {
        self.variables.push(SwitchVariable::new(key, value));
    }

    pub fn remove_variable<S: Into<String>>(&mut self, key: S) -> bool {
        let key = key.into();
        for (idx, variable) in self.variables.iter().enumerate() {
            if variable.key == key {
                self.variables.remove(idx);
                return true;
            }
        }
       
        false
    }

    pub fn get_variable<S: Into<String>>(&self, key: S) -> Option<&SwitchVariable> {
        let key = key.into();
        for variable in self.variables.iter() {
            if variable.key == key {
                return Some(variable);
            }
        }

        None
    }
}