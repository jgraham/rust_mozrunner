use std::process;
use std::process::{Command, Stdio};
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

use mozprofile::profile::Profile;
use mozprofile::preferences::FIREFOX_PREFERENCES;

pub trait Runner {
    fn start(&mut self) -> IoResult<()>;

    fn build_command(&self, &mut Command);

    fn is_running(&self) -> bool;

    fn stop(&mut self) -> IoResult<Option<process::ExitStatus>>;
}

pub struct FirefoxRunner {
    pub binary: PathBuf,
    args: Vec<String>,
    process: Option<process::Child>,
    pub ret_code: Option<process::ExitStatus>,
    pub profile: Profile
}

impl FirefoxRunner {
    pub fn new(binary: &Path, profile: Option<Profile>) -> IoResult<FirefoxRunner> {
        let prof = match profile {
            Some(p) => p,
            None => try!(Profile::new(None))
        };

        Ok(FirefoxRunner {
            binary: binary.to_path_buf(),
            process: None,
            ret_code: None,
            args: Vec::new(),
            profile: prof
        })
    }
}

impl Runner for FirefoxRunner {
    fn start(&mut self) -> IoResult<()> {
        let mut cmd = Command::new(&self.binary);
        self.build_command(&mut cmd);

        self.profile.preferences.insert_vec(&FIREFOX_PREFERENCES);

        try!(self.profile.write_prefs());

        let process = try!(cmd.spawn());
        self.process = Some(process);
        Ok(())
    }

    fn build_command(&self, command: &mut Command) {
        command.args(&self.args[..]).arg("--marionette").arg("--profile").arg(&self.profile.path)
            .stdout(Stdio::inherit()).stderr(Stdio::inherit())
            .env("MOZ_NO_REMOTE", "1").env("NO_EM_RESTART", "1");
    }

    fn is_running(&self) -> bool {
        self.process.is_some() && self.ret_code.is_none()
    }

    fn stop(&mut self) -> IoResult<Option<process::ExitStatus>> {
        match self.process.as_mut() {
            Some(p) => {
                try!(p.kill());
                let status = try!(p.wait());
                self.ret_code = Some(status);
            },
            None => {}
        };
        Ok(self.ret_code)
    }
}
