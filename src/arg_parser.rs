use clap::Command;

pub fn parse_args() {
    Command::new("wallsd")
        .version("v0.0.0")
        .about(clap::crate_description!())
        .get_matches();
}
