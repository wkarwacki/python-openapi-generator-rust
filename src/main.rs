use clap::Parser;

extern crate trust;

use trust::do_main;
use trust::Cli;

fn main() {
    let cli = Cli::parse();
    do_main(cli);
}
