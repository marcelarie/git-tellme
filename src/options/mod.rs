use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "n", long = "notifications")]
    /// Github user notifications
    notifications: bool,
}
