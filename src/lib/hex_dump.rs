use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::process::{Command, Stdio};
use thiserror::Error;

const MAX_LEN: usize = 40;

#[derive(Error, Debug)]
pub enum HexDumpError {
    #[error("Input too big {0} - MAX {}", MAX_LEN)]
    TooBig(usize),

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

pub fn hex_dump_via_od(s: &str, cmd: &str) -> Result<String, HexDumpError> {
    if s.len() > MAX_LEN {
        return Err(HexDumpError::TooBig(s.len()));
    }

    fake_net_error(true)?;
    fake_os_error(false)?;

    let mut cmd = Command::new(cmd)
        .args(["-A", "x", "-t", "x1z", "-v"])
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

    let output = cmd.wait_with_output()?;

    let mut result = String::new();

    output.stdout.as_slice().read_to_string(&mut result)?;

    Ok(result)
}
