use structopt::StructOpt;

// TODO: Think alternative to `-u` user parameter
#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "n", long = "get-notifications")]
    /// Get Github user notifications
    #[allow(dead_code)]
    pub notifications: bool,

    #[structopt(short = "f", long = "--system")]
    /// Show notifications on the system
    pub system: bool,

    #[structopt(short = "r", long = "repos")]
    /// Get Github user repositories
    pub repos: bool,

    #[structopt(
        short = "s",
        long = "subscribe",
        help = "Pass `-su` and the username you want to subscribe to"
    )]
    // Subscribe to user profile
    pub sub: bool,

    #[structopt(
        short = "Un",
        long = "unsubscribe",
        help = "Pass `-Unu` and the username you want to unsubscribe to"
    )]
    // Unsubscribe to user profile
    pub unsub: bool,

    #[structopt(short = "u", long = "user")]
    /// Select specific user profile
    pub user:         Option<String>,
    #[structopt(
        short = "t",
        long = "token",
        help = "Pass `-t` and you're GitHub token from\nhttps://github.com/settings/tokens"
    )]
    /// Save user GitHub Auth Token
    pub github_token: Option<String>,
}
