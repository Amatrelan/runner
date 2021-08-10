use indexmap::IndexMap;
use log::debug;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use std::collections::HashMap;
use toml;

fn bool_default() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Global {
    log_level: String,
    #[serde(default = "bool_default")]
    dry_run: bool,
}

impl Default for Global {
    fn default() -> Self {
        Global {
            log_level: "debug".to_string(),
            dry_run: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    args: Option<Vec<String>>,

    #[serde(flatten)]
    named_args: HashMap<String, String>,
}

impl Default for Task {
    fn default() -> Self {
        let mut hmap: HashMap<String, String> = HashMap::new();
        hmap.insert("example".to_string(), "value".to_string());

        Self {
            args: None,
            named_args: hmap,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    global: Global,

    task: IndexMap<String, Task>,
}

impl Default for Config {
    fn default() -> Self {
        let mut imap: IndexMap<String, Task> = IndexMap::new();

        imap.insert("example_task".to_string(), Task::default());

        Self {
            global: Global::default(),
            task: imap,
        }
    }
}

pub fn deserialize_config(config: String) {
    let config: Config = toml::from_str(config.as_str()).unwrap();
    debug!("PRINTING CONFIG OUTPUT\n{:#?}", config);
}

pub fn generate_default_config(output_path: String) -> String {
    let config: Config = Config::default();

    debug!("Example config:\n{:#?}", config);

    todo!()
}
