extern crate event;
extern crate syscall;

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::process;
use std::str::FromStr;
use std::time::Instant;

mod packets;
use packets::*;

static RHEINFALL_MAN: &str = /* @MANSTART{rheinfall} */ r#"
NAME
    rheinfall - forward or generate network packets
SYNOPSIS
    rheinfall [-h | --help] [-g | --generate] [-s | --size packetsize]
DESCRIPTION
    rheinfall receives and transmits raw ethernet frames via the Redox
    network scheme, i.e. directly through the driver (bypassing other parts
    of the network stack). Default operation mode is mirroring, i.e. all
    received packets are sent back without modification. It is also possible
    to run rheinfall as a packet generator with rheinfall --generate.
OPTIONS
    -g
    --generate
        Generate network packets.
    -h
    --help
        Print this manual page.
    -s
    --size
        Size of generated packets, either 60 or 1500 Bytes. Defaults to 1500 Bytes.
"#; /* @MANEND */

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);
    let mut mirror = true;
    let mut size = 1500;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{}", RHEINFALL_MAN);
                return Ok(());
            }
            "--generate" | "-g" => {
                mirror = false;
            }
            "--size" | "-s" => {
                size = i64::from_str(&args.next().expect("No argument to -s option"))
                    .expect("Invalid argument to -s option");
                if !(size == 60 || size == 1500) {
                    return Err("Size has to be 60 or 1500 Bytes".into());
                }
            }
            _ => {}
        }
    }

    let network_path = "network:".to_string();

    let network_fd = syscall::open(&network_path, syscall::O_RDWR | syscall::O_NONBLOCK)
        .unwrap_or_else(|_| panic!("Can't open path {}", network_path));

    let mut network_file = unsafe { File::from_raw_fd(network_fd as RawFd) };

    let mut time = Instant::now();
    let mut counter = 0;
    let mut packets = 0;

    if mirror {
        loop {
            let mut payload = [0u8; 2048];

            match network_file.read(&mut payload) {
                Ok(read) if read >= 60 => {
                    network_file.write_all(&payload[..read])?;
                    packets += 1;
                }
                _ => (),
            }

            // don't poll the time unnecessarily
            if counter & 0xfff == 0 {
                let elapsed = time.elapsed();
                let nanos = elapsed.as_secs() as u32 * 1_000_000_000 + elapsed.subsec_nanos();
                // every second
                if nanos > 1_000_000_000 {
                    println!("Forwarded packets: {}/s", packets);

                    time = Instant::now();
                    packets = 0;
                }
            }

            counter += 1;
        }
    } else if size == 60 {
        loop {
            network_file.write_all(&SMALL_PACKET)?;
            packets += 1;

            // don't poll the time unnecessarily
            if counter & 0x8f == 0 {
                let elapsed = time.elapsed();
                let nanos = elapsed.as_secs() as u32 * 1_000_000_000 + elapsed.subsec_nanos();
                // every second
                if nanos > 1_000_000_000 {
                    println!("Sent packets: {}/s", packets);

                    time = Instant::now();
                    packets = 0;
                }
            }

            counter += 1;
        }
    } else {
        loop {
            network_file.write_all(&STANDARD_PACKET)?;
            packets += 1;

            // don't poll the time unnecessarily
            if counter & 0x8f == 0 {
                let elapsed = time.elapsed();
                let nanos = elapsed.as_secs() as u32 * 1_000_000_000 + elapsed.subsec_nanos();
                // every second
                if nanos > 1_000_000_000 {
                    println!("Sent packets: {}/s", packets);

                    time = Instant::now();
                    packets = 0;
                }
            }

            counter += 1;
        }
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
