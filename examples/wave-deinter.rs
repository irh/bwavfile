//! wave-inter.rs
//! (c) 2021 Jamie Hardt. All rights reserved.
//! 
//! This program demonstrats combining several wave files into a single
//! polyphonic wave file.

use std::io;
use std::path::Path;

extern crate bwavfile;
use bwavfile::{Error,WaveReader, ChannelDescriptor, ChannelMask};

#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn name_suffix(force_numeric : bool, delim : &str, index: usize, channel_descriptor : &ChannelDescriptor) -> String {
    if force_numeric || channel_descriptor.speaker == ChannelMask::DirectOut {
        format!("{}A{:02}", delim, index)
    } else {
        let chan_name = match channel_descriptor.speaker {
            ChannelMask::FrontLeft => "L",
            ChannelMask::FrontCenter => "C",
            ChannelMask::FrontRight => "R",
            ChannelMask::BackLeft => "Ls",
            ChannelMask::BackRight => "Rs",
            ChannelMask::BackCenter => "S",
            ChannelMask::TopCenter => "Tc",
            ChannelMask::LowFrequency => "Lfe",
            ChannelMask::SideLeft => "Lss",
            ChannelMask::SideRight => "Rss",
            ChannelMask::FrontCenterLeft => "Lc",
            ChannelMask::FrontCenterRight => "Rc",
            ChannelMask::TopFrontLeft => "Ltf",
            ChannelMask::TopFrontCenter => "Ctf",
            ChannelMask::TopFrontRight => "Rtf",
            ChannelMask::TopBackLeft => "Ltb",
            ChannelMask::TopBackCenter => "Ctb",
            ChannelMask::TopBackRight => "Rtb",
            ChannelMask::DirectOut => panic!("Error, can't get here")
        };
        format!("{}{}", delim, chan_name)
    }
}

fn process_file(infile: &str, delim : &str, numeric_channel_names : bool) -> Result<(), Error> {
    let mut read_wave = WaveReader::open(infile)?;
    let channel_desc = read_wave.channels()?;
    
    if channel_desc.len() == 1 {
        println!("Input file in monoaural, exiting.");
        return Ok(());
    }

    let infile_path = Path::new(infile);
    let basename = infile_path.file_stem().expect("Unable to extract file basename").to_str().unwrap();
    let output_dir = infile_path.parent().expect("Unable to derive parent directory");

    for (n, channel) in channel_desc.iter().enumerate() {
        let suffix = name_suffix(numeric_channel_names, delim, n + 1, channel);
        let outfile_name = output_dir.join(format!("{}{}.wav", basename, suffix)).into_os_string().into_string().unwrap();
        println!("Will create file {}", outfile_name);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("wave-deinter")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Extract each channel of a polyphonic wave file as a new monoaural wave file.")
        .arg(Arg::with_name("numeric_names")
            .long("numeric")    
            .short("n")
            .help("Use numeric channel names \"01\" \"02\" etc.")
            .takes_value(false)
        )
        .arg(Arg::with_name("channel_delimiter")
            .long("delim")
            .short("d")
            .help("Channel label delimiter.")
            .default_value(".")
        )
        .arg(Arg::with_name("INPUT")
            .help("Input wave file")
            .required(true)
            .multiple(true)
        )
        .get_matches();
    
    let delimiter = matches.value_of("channel_delimiter").unwrap();
    let use_numeric_names = matches.is_present("numeric_names");
    let infile = matches.value_of("INPUT").unwrap();

    match process_file(infile, delimiter, use_numeric_names) {
        Err(Error::IOError(io)) => Err(io),
        Err(e) => panic!("Error: {:?}", e),
        Ok(()) => Ok(())
    }
}