use config::{Config, ConfigError, Environment, File};
use garde::Validate;
use serde_derive::Deserialize;
use url::Url;

fn is_valid_uri<T>(value: &str, _: T) -> garde::Result {
    match Url::parse(value) {
        Ok(_) => Ok(()),
        Err(e) => Err(garde::Error::new(e.to_string())),
    }
}

#[derive(Debug, Default, Deserialize, Validate)]
pub struct Settings {
    #[garde(skip)]
    pub env: String,
    #[garde(custom(is_valid_uri))]
    pub database_uri: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let conf = Config::builder()
            // use settings from local.ini
            .add_source(File::with_name("local").required(false))
            // take any env var with APP_*
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        let conf = conf.try_deserialize::<Settings>().unwrap();
        match conf.validate(&()) {
            Ok(_) => Ok(conf),
            Err(e) => Err(ConfigError::Message(e.to_string())),
        }
    }
}
