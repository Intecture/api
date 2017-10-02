// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Endpoint for running shell commands.

pub mod providers;

use erased_serde::Serialize;
use errors::*;
use ExecutableProvider;
use host::Host;
use self::providers::{Nix, NixRemoteProvider};

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub struct Command {
    shell: Vec<String>,
    cmd: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub trait CommandProvider<'a> {
    fn available(&Host) -> bool where Self: Sized;
    fn try_new(&'a Host, &[&str], Option<&[&str]>) -> Option<Self> where Self: Sized;
    fn exec(&self) -> Result<CommandResult>;
}

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub enum RemoteProvider {
    Nix(NixRemoteProvider)
}

impl <'de>ExecutableProvider<'de> for RemoteProvider {
    fn exec(self, host: &Host) -> Result<Box<Serialize>> {
        match self {
            RemoteProvider::Nix(p) => p.exec(host)
        }
    }
}

pub fn factory<'a>(host: &'a Host, cmd: &[&str], shell: Option<&[&str]>) -> Result<Box<CommandProvider<'a> + 'a>> {
    if let Some(p) = Nix::try_new(host, cmd, shell) {
        Ok(Box::new(p))
    } else {
        Err(ErrorKind::ProviderUnavailable("Command").into())
    }
}
