use clap::Parser;
use shadow_rs::shadow;

const DEFAULT_DATABASE: &'static str = "sqlite:///hangitbot.db";
const DEFAULT_API_URL: &'static str = "https://api.telegram.org";

shadow!(build);

#[derive(Parser, Debug)]
#[clap(author, version=build::TAG, about, long_about = None)]
pub struct Args {
    /// Enable debug mode
    #[clap(short = 'D', long, value_parser, default_value_t = false)]
    pub debug: bool,

    /// Telegram bot token
    #[clap(short, long, value_parser, env = "TGBOT_TOKEN")]
    pub tgbot_token: String,

    /// Database URI
    #[clap(short, long, value_parser, env = "DATABASE_URI", default_value=DEFAULT_DATABASE)]
    pub database_uri: String,

    /// Api Server URL
    #[clap(long, value_parser, env = "API_URL", default_value=DEFAULT_API_URL)]
    pub api_url: String,

    /// GroupID blacklisted
    #[clap(short = 'b', long, value_parser, env = "GROUP_BANNED", value_delimiter = ',', num_args = 1..)]
    pub group_banned: Vec<i64>
}
