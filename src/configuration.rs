/*##############################################
#Ce fichiers sert a loader le configuration.yaml
#La configuration doit etre loader dans main.rs
#
###############################################*/


#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError>{
    //Initialiser configuration Reader
    let settings = config::Config::builder()
        .add_source(
            config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    //Maintenant on popule notre configuration type (struct::Setting) avec config file
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings{
    pub fn connection_string(&self)->String{
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}