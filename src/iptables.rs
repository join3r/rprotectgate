use crate::{anyhow, Result};
use std::{net::SocketAddr, process::Command, str::FromStr, ops::Deref, sync::RwLock};

const IPTABLES: &str = "/usr/sbin/iptables";
const CHAIN_NAME: &str = "rprotectgate";

#[derive(Clone)]
pub struct AllowedList {
    inner: Vec<Account>,
}

#[derive(Clone)]
struct Account {
    ip: SocketAddr,
    username: Username,
}

#[derive(Clone)]
struct Username {
    inner: String,
}

impl AllowedList {
    pub fn new() -> Result<AllowedList> {
        let allowed_list = AllowedList {
            inner: Vec::new(),
        };
        
        // add table ip filter
        // add chain ip filter rprotectgate { type filter hook input priority 0; policy drop; }
        // add rule ip filter rprotectgate ip saddr 192.168.100.1 counter accept

        let _res = execute([IPTABLES, "-F", CHAIN_NAME].as_ref())?; // TODO: only if CHAIN_NAME exists
        let _res = execute([ IPTABLES,"-X", CHAIN_NAME ].as_ref())?;
        let _res = execute([ IPTABLES, "-N", CHAIN_NAME ].as_ref())?;
        allowed_list.update()?;
        let _res = execute([IPTABLES, "-A", CHAIN_NAME, "-j", "DROP"].as_ref())?;
        Ok(allowed_list)
    }
    pub fn update(&self) -> Result<()> {
        if let Some(ips) = self.get_ips() {
            let _res = execute([IPTABLES, "-I", CHAIN_NAME, "-s", &ips, "-j", "ACCEPT"].as_ref())?;
        }
        Ok(())
    }
    pub fn add(&mut self, ip: SocketAddr, username: String) -> Result<()> {
        self.inner.push(Account {
            ip, username: username.parse()?
        });
        Ok(())
    }
    pub fn get_ips(&self) -> Option<String> {
        if !self.inner.is_empty() {
            let p = self.inner.iter().map(|acc| acc.username.deref().clone()).collect::<Vec<_>>().join(",");
            Some(p)
        } else {
            None
        }
    }
}

impl FromStr for Username {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.len() {
            3..=12 => Ok(Username { inner: s.to_string() }),
            _ => Err(anyhow!("Username must be between 3 and 12 characters long"))
        }
    }
}

impl Deref for Username {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// execute command
fn execute(args: &[&str]) -> Result<String> {
    let command = args[0];
    let args = args[1..].iter();
    let output = Command::new(command).args(args).output()?;
    match output.status.success() {
        true => Ok(String::from_utf8(output.stdout)?),
        false => {
            let stdout =
                String::from_utf8(output.stdout).unwrap_or("invalid UTF8 in stdout".to_string());
            let stderr =
                String::from_utf8(output.stderr).unwrap_or("invalid UTF8 in stderr".to_string());
            Err(anyhow!("{}\n{}", stdout, stderr))
        }
    }
}
