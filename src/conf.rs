use confy::load;
use serde::{Deserialize,Serialize};

// Define structs representing the sections in the TOML file
#[derive(Default, Debug, Deserialize, Serialize)]
struct Config {
    config: ConfigSection,
    deps: DepsSection,
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct ConfigSection {
    name: String,
    pass: String,
}

#[derive(Default, Debug, Deserialize,Serialize)]
struct DepsSection {
    dep1: String,
    dep2: String,
}

pub fn lets_conf(){
    // Load the configuration for the "mdm" CLI program
    let config: Config = load("mdm",None).unwrap();

    // Access the fields of each section
    println!("Config Section:");
    println!("Name: {}", config.config.name);
    println!("Password: {}", config.config.pass);

    println!("\nDeps Section:");
    println!("Dep1: {}", config.deps.dep1);
    println!("Dep2: {}", config.deps.dep2);

}
