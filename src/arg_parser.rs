use clap::Command;
use lazy_static::lazy_static;
use crate::build;

pub fn parse_args() {
    lazy_static! {
        static ref PKG_VERSION: String = format!("v{}-{} ({}), git workspace was {}", build::PKG_VERSION, build::COMMIT_HASH, build::BUILD_RUST_CHANNEL, is_git_workspace_clean());
    }

    Command::new("wallsd")
        .version(PKG_VERSION.as_str())
        .about(clap::crate_description!())
        .get_matches();
}

fn is_git_workspace_clean() -> String {
    if build::GIT_CLEAN {
        "clean".to_string()
    } else {
        "dirty".to_string()
    }
}
