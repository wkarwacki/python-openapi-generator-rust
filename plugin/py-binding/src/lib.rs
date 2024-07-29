use pyo3::prelude::*;

use clap::Parser;

use trust::Cli;
use trust::do_main;

#[pyfunction]
fn main(args: Vec<String>) {
    let cli = Cli::parse_from(args);
    do_main(cli);
}

#[pymodule]
fn trustspecpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(main, m)?)?;
    Ok(())
}
