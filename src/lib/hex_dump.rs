use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::process::{Command, Stdio};
use thiserror::Error;

const MAX_LEN: usize = 100;

#[derive(Error, Debug)]
pub enum HexDumpError {
    #[error("Input too big {0} - MAX {}", MAX_LEN)]
    TooBig(usize),

    #[error("Command failed - exit_status {0}")]
    ErrorExit(i32),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Net(#[from] std::net::AddrParseError),
}

fn fake_os_error(fail: bool) -> Result<u32, io::Error> {
    if fail {
        Err(io::Error::from_raw_os_error(123))
    } else {
        Ok(100)
    }
}

fn fake_net_error(fail: bool) -> Result<IpAddr, std::net::AddrParseError> {
    if fail {
        let addr: IpAddr = "127.0.0.1:8080".parse()?;

        Ok(addr)
    } else {
        Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    }
}

fn check_len(s: &str) -> Result<(), HexDumpError> {
    if s.len() > MAX_LEN {
        return Err(HexDumpError::TooBig(s.len()).into());
    }

    Ok(())
}

pub fn hex_dump_via_cmd(s: &str, cmd: &str, args: &[&str]) -> Result<String, HexDumpError> {
    // Some potential errors
    check_len(s)?;
    fake_net_error(false)?;
    fake_os_error(false)?;

    let mut cmd = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = cmd.stdin.take().expect("Failed to get stdin");

    let s2 = s.to_string();

    std::thread::spawn(move || {
        stdin
            .write_all(s2.as_bytes())
            .expect("Failed to write to input");
    });

    let mut result = String::new();
    let just_output = false;

    if just_output {
        let output = cmd.wait_with_output()?;

        output.stdout.as_slice().read_to_string(&mut result)?;
    } else {
        let mut stdout = cmd.stdout.take().expect("Failed to get stdout");

        let status = cmd.wait()?;
        let code = status.code().unwrap_or(-1);

        if code != 0 {
            return Err(HexDumpError::ErrorExit(code));
        }

        stdout.read_to_string(&mut result)?;
    }

    Ok(result)
}

pub fn hex_dump_via_cmd_anyh(s: &str, cmd: &str, args: &[&str]) -> anyhow::Result<String> {
    Ok(hex_dump_via_cmd(s, cmd, args)?)
}
