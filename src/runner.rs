use std::io::{process, IoResult};
use std::path::Path;

use mozprofile::profile::Profile;
use mozprofile::preferences::FIREFOX_PREFERENCES;

pub trait Runner {
    fn command(&self) -> process::Command;

    fn start(&mut self) -> IoResult<()>;

    fn is_running(&self) -> bool;

    fn stop(&mut self) -> IoResult<Option<process::ProcessExit>>;
}

pub struct FirefoxRunner {
    pub binary: Path,
    args: Vec<String>,
    process: Option<process::Process>,
    pub ret_code: Option<process::ProcessExit>,
    pub profile: Profile
}

impl FirefoxRunner {
    pub fn new(binary: Path, profile: Option<Profile>) -> IoResult<FirefoxRunner> {
        let prof = match profile {
            Some(p) => p,
            None => try!(Profile::new(None))
        };

        Ok(FirefoxRunner {
            binary: binary,
            process: None,
            ret_code: None,
            args: Vec::new(),
            profile: prof
        })
    }
}

impl Runner for FirefoxRunner {
    fn command(&self) -> process::Command {
        // TODO: Make sure [-]-foreground is the last arg if present and is
        // always present on OSX
        process::Command::new(self.binary.clone()).arg("--profile").arg(self.profile.path.clone()).args(self.args.as_slice()).clone()
    }

    fn start(&mut self) -> IoResult<()> {
        let mut cmd = self.command();
        //XXX .clone seems unnecessary here
        cmd = cmd.env("MOZ_NO_REMOTE", "1").env("NO_EM_RESTART", "1").clone();

        self.profile.preferences.insert_vec(&FIREFOX_PREFERENCES);

        self.profile.write_prefs();

        let process = try!(cmd.spawn());
        self.process = Some(process);
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.process.is_some() && self.ret_code.is_none()
    }

    fn stop(&mut self) -> IoResult<Option<process::ProcessExit>> {
        match self.process.as_mut() {
            Some(p) => {
                try!(p.signal_kill());
                let status = try!(p.wait());
                self.ret_code = Some(status);
            },
            None => {}
        }
        Ok(self.ret_code)
    }
}
