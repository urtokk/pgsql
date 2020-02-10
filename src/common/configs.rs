extern crate config;
extern crate serde;

use std::net::IpAddr;
use std::collections::HashMap;
use config::{
    Config,
    ConfigError,
    File
};

/// Struct to manage entries with connection information to a database
#[derive(Debug, Deserialize)]
pub struct Entry {
    flavor: String,
    address: IpAddr,
    port: u16,
    user: String,
    password: String,
    database: String
}

/// postgrsql settings
// TODO: integrate client options
#[derive(Debug, Deserialize)]
pub struct postgresql {
    binary: String
}

/// Struct to manage configuration file with global configurations and database entries
#[derive(Debug, Deserialize)]
pub struct settings{
    postgresql: postgresql,
    entries: HashMap<String, Entry>,    
}

impl settings{

}