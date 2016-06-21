#[macro_use]
extern crate log;
extern crate mozprofile;
#[cfg(target_os = "windows")]
extern crate winreg;

pub mod runner;

pub use runner::platform::firefox_default_path;

#[cfg(test)]
mod test {
    use runner::Runner;

    use std::path::Path;

    #[test]
    fn it_works() {
        let path = Path::new("/home/jgraham/develop/gecko/obj-x86_64-unknown-linux-gnu/dist/bin/firefox/").unwrap();
        let mut fx_runner = runner::FirefoxRunner::new(path, None).unwrap();
        fx_runner.start();
    }
}
