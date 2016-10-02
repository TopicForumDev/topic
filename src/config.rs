use std::default::Default;
use toml::{Parser,Value,decode};
use iron::typemap;

use std::fs::File;
use std::io::Read;

#[derive(RustcDecodable,Debug)]
pub struct Config {
    pub database: DbConfig,
    pub server: ServerConfig,
    pub site: SiteConfig,
}

#[derive(RustcDecodable,Debug)]
pub struct DbConfig {
    pub db: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

#[derive(RustcDecodable,Debug)]
pub struct ServerConfig {
    pub admin_pwd: String,
    pub host: String,
    pub port: u16,
}

#[derive(RustcDecodable,Debug)]
pub struct SiteConfig {
    pub name: String,
    pub posts_per_page: u64,
    pub threads_per_page: u64,
}

impl Default for SiteConfig {
    fn default() -> Self {
        SiteConfig {
            name: "MySite".to_string(),
            posts_per_page: 10,
            threads_per_page: 20,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            admin_pwd: "asa9310".to_string(),
            host: "localhost".to_string(),
            port: 80,
        }
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            user: "postgres".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            password: "".to_string(),
            db: "postgres".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig::default(),
            site: SiteConfig::default(),
            database: DbConfig::default(),
        }
    }
}

impl typemap::Key for Config {
    type Value = Self;
}

pub fn parse(path: &str) -> Config {
    let mut config = String::new();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e)  => {
            panic!("Could not load config from {}: {:?}", path, e);
        }
    };

    file.read_to_string(&mut config).unwrap();

    let mut parser = Parser::new(&config);
    let toml = parser.parse();
    if let None = toml {
        println!("Error while parsing config file {}", path);
        for err in &parser.errors {
            let (ll, lc) = parser.to_linecol(err.lo);
            let (hl, hc) = parser.to_linecol(err.hi);
            println!("\t{}:{} - {}:{} : {}", ll, lc, hl, hc, err.desc);
        }
        panic!("Failed to load config, quitting!");
    }
    let toml = toml.unwrap();
    let decoded: Option<Config> = decode(Value::Table(toml));
    match decoded {
        Some(x) => x,
        None => panic!(format!("Failed to deserialize when loading config file {}. Are all the fields present?", path))
    }
}
