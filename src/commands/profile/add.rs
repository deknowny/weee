use clap;


#[derive(Debug, clap::Args)]
pub struct Add {
    #[clap(required = true)]
    profile_name: String
}
