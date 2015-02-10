extern crate mozprofile;

use runner::Runner;

use std::path::Path;
use std::str::FromStr;

pub mod runner;

#[test]
fn it_works() {
    let path: Path = FromStr::from_str("/home/jgraham/develop/gecko/obj-x86_64-unknown-linux-gnu/dist/bin/firefox/").unwrap();
    let mut fx_runner = runner::FirefoxRunner::new(path, None).unwrap();
    fx_runner.start();
}
