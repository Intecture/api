// Copyright 2015-2017 Intecture Developers.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use command::{Command, CommandProvider, CommandResult};
use erased_serde::Serialize;
use errors::*;
use ExecutableProvider;
use host::*;
use std::process;

const DEFAULT_SHELL: [&'static str; 2] = ["/bin/sh", "-c"];

pub struct Nix<'a> {
    host: &'a Host,
    inner: Command
}

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub enum RemoteProvider {
    Available,
    Exec(Command),
}

impl <'de>ExecutableProvider<'de> for RemoteProvider {
    fn exec(self, host: &Host) -> Result<Box<Serialize>> {
        match self {
            RemoteProvider::Available => Ok(Box::new(Nix::available(host))),
            RemoteProvider::Exec(inner) => {
                let p = Nix { host, inner };
                Ok(Box::new(p.exec()?))
            }
        }
    }
}

impl <'a>CommandProvider<'a> for Nix<'a> {
    fn available(host: &Host) -> bool {
        if host.is_local() {
            cfg!(not(windows))
        } else {
            unimplemented!();
            // let r = RemoteProvider::Available;
            // self.host.send(r).chain_err(|| ErrorKind::RemoteProvider("Command", "available"))?;
            // Ok(self.host.recv()?)
        }
    }

    fn try_new(host: &'a Host, cmd: &[&str], shell: Option<&[&str]>) -> Option<Nix<'a>> {
        if Self::available(host) {
            let inner = Command {
                shell: shell.unwrap_or(&DEFAULT_SHELL).to_owned().iter().map(|s| s.to_string()).collect(),
                cmd: cmd.to_owned().iter().map(|s| s.to_string()).collect(),
            };
            Some(Nix { host, inner })
        } else {
            None
        }
    }

    fn exec(&self) -> Result<CommandResult> {
        if self.host.is_local() {
            let (shell, shell_args) = self.inner.shell.split_first()
                                                      .ok_or("Invalid shell provided")?;
            let out = process::Command::new(shell)
                                       .args(shell_args)
                                       .args(&self.inner.cmd)
                                       .output()
                                       .chain_err(|| "Command execution failed")?;
            Ok(CommandResult {
                success: out.status.success(),
                exit_code: out.status.code(),
                stdout: out.stdout,
                stderr: out.stderr
            })
        } else {
            unimplemented!();
            // let r = RemoteProvider::Load;
            // self.host.send(r).chain_err(|| ErrorKind::RemoteProvider("Command", "exec"))?;
            // let result: CommandResult = self.host.recv()?;
            // Ok(result)
        }
    }
}
