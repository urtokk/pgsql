use std::net::IpAddr;

pub enum Flavor {
    PostgreSQL(String)
}

pub struct Entry {
    name: String,
    flavor: Flavor,
    address: IpAddr,
    port: u16,
    user: String,
    password: String,
    database: String
}