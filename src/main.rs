use clap::Parser;

extern crate trust;

use trust::{do_main, Cli};

fn main() {
    let cli = Cli::parse();
    do_main(cli);
}
