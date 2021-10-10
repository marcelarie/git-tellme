use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "n", long = "get-notifications")]
    /// Get Github user notifications
    pub notifications: bool,

    #[structopt(short = "f", long = "--system")]
    /// Show notifications on the system
    pub system: bool,

    #[structopt(short = "r", long = "repos")]
    /// Get Github user repositories
    pub repos: bool,

    #[structopt(short = "sub", long = "subscribe")]
    // Subscribe to user profile
    pub sub: bool,

    #[structopt(short = "u", long = "user")]
    /// Select specific user profile
    pub user: Option<String>,
}
