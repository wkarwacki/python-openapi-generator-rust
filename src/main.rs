use clap::Parser;

extern crate trust;

use trust::Cli;
use trust::do_main;

fn main() {
    let cli = Cli::parse();
    do_main(cli);
}

