#[derive(clap::Parser, Debug)]
pub struct Config {
    #[clap(long, env = "DATABASE_URL")]
    pub db_url: String,
}


