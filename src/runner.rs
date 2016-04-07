use std::process;
use std::process::{Command, Stdio};
use std::io::{Result as IoResult, Error as IoError};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::convert::From;
use std::fmt;

use mozprofile::profile::Profile;
use mozprofile::prefdata::FIREFOX_PREFERENCES;
use mozprofile::prefreader::PrefReaderError;

pub trait Runner {
    fn start(&mut self) -> Result<(), RunnerError>;

    fn build_command(&self, &mut Command);

    fn is_running(&self) -> bool;

    fn stop(&mut self) -> IoResult<Option<process::ExitStatus>>;
}

#[derive(Debug)]
pub enum RunnerError {
    Io(IoError),
    PrefReader(PrefReaderError)
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for RunnerError {
    fn description(&self) -> &str {
        match *self {
            RunnerError::Io(ref err) => err.description().clone(),
            RunnerError::PrefReader(ref err) => err.description().clone(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RunnerError::Io(ref err) => err.cause(),
            RunnerError::PrefReader(ref err) => err.cause(),
        }
    }
}

impl From<IoError> for RunnerError {
    fn from(value: IoError) -> RunnerError {
        RunnerError::Io(value)
    }
}

impl From<PrefReaderError> for RunnerError {
    fn from(value: PrefReaderError) -> RunnerError {
        RunnerError::PrefReader(value)
    }
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
    fn start(&mut self) -> Result<(), RunnerError> {
        let mut cmd = Command::new(&self.binary);
        self.build_command(&mut cmd);

        let mut prefs = try!(self.profile.user_prefs());
        prefs.insert_slice(&FIREFOX_PREFERENCES[..]);

        try!(prefs.write());

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
