use clap;

#[derive(Debug, clap::Args)]
pub struct Remove {
    #[clap(required = true)]
    profile_name: String,
}
