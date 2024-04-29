use clap::Command;
use lazy_static::lazy_static;
use crate::build;

pub fn parse_args() {
    lazy_static! {
        static ref PKG_VERSION: String = format!("v{}-{}", build::PKG_VERSION, build::COMMIT_HASH);
    }

    Command::new("wallsd")
        .version(PKG_VERSION.as_str())
        .about(clap::crate_description!())
        .get_matches();
}
