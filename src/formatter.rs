//             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2018 Thomas Bailleux <thomas@bailleux.me>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.
//
// Author: zadig <thomas chr(0x40) bailleux.me>

use std;
use ansi_term;
use libprefetch;
use clap;
use chrono;


const DIF_TIME_WINDOWS: u64 = 116444736000000000u64;

pub(crate) struct Formatter {
  color: bool,
  metrics: bool,
  verbose: bool,
  volumes: bool
}

impl Formatter {
  pub(crate) fn new() -> Formatter {
    Formatter {
      color: false,
      metrics: false,
      verbose: false,
      volumes: false
    }
  }

  pub(crate) fn format(&self, name: &str, date: &str, counter: &str)
      -> std::string::String {
    match self.color {
      false => format!("{:<26} {:^21} {:>18}", name, date, counter),
      true  => format!("{:<36} {:^19} {:>28}", name, date, counter)
    }
  }

  pub fn format_name<'a>(&self, name: &'a str) -> std::borrow::Cow<'a, str> {
    match self.color {
      false => std::borrow::Cow::Borrowed(name),
      true => std::borrow::Cow::Owned(
          ansi_term::Colour::Green.paint(name).to_string())
    }
  }

  pub fn format_counter<'a>(&self, counter: &'a str) -> std::borrow::Cow<'a, str> {
    match self.color {
      false => std::borrow::Cow::Borrowed(counter),
      true => std::borrow::Cow::Owned(
          ansi_term::Colour::Green.bold().paint(counter).to_string())
    }
  }

  pub(crate) fn print_prefix(&self) {
    eprintln!("{}", self.format(
      &self.format_name("Executable name"),
      &"Last execution time",
      &self.format_counter("Execution counter")));
  }

  pub(crate) fn set_options(&mut self, matches: &clap::ArgMatches) {
    self.color = matches.is_present("color");
    self.metrics = matches.is_present("metrics");
    self.verbose = matches.is_present("verbose");
    self.volumes = matches.is_present("volumes");
  }

  pub(crate) fn print(&self, pf: &libprefetch::Prefetch) {
    println!("{}", self.format(
      &self.format_name(pf.name()).into_owned(),
      &self.format_date(pf.last_execution_time()),
      &self.format_counter(&pf.execution_counter().to_string())));
    if self.metrics {
      if let Ok(metrics) = pf.metrics() {
        let metrics: std::vec::Vec<&libprefetch::metric::MetricEntry>
            = metrics.collect();
        println!("└─Metrics:");
        for i in 0 .. metrics.len() - 1 {
          self.print_metric("├", metrics[i], false);
        }
        self.print_metric("└", metrics.last().unwrap(), true);
        println!("");
      } else {
        eprintln!("Metrics aren't available.");
      }
    }
    if self.volumes {
      if let Ok(volumes) = pf.volumes() {
        let volumes: std::vec::Vec<&libprefetch::volume::VolumeEntry>
            = volumes.collect();
        println!("└─Volumes:");
        for i in 0 .. volumes.len() - 1 {
          self.print_volume("├", volumes[i], false);
        }
        self.print_volume("└", volumes.last().unwrap(), true);
        println!("")
      } else {
        eprintln!("Volumes aren't available.");
      }
    }
  }

  fn format_date(&self, date: u64) -> std::string::String {
    use chrono::TimeZone;
    let result: std::string::String;
    if date == 0 {
      result = std::string::String::from("(no MAC specified)");
    } else {
      result = chrono::Utc.timestamp(((date - DIF_TIME_WINDOWS)
          / 10000000) as i64, 0).format("%Y-%m-%d %H:%M:%S").to_string();
    }

    match self.color {
      false => result,
      true => ansi_term::Colour::Yellow.paint(result).to_string()
    }
  }

  fn print_metric(&self, branch: &str, metric:
    &libprefetch::metric::MetricEntry, last: bool) {
    if !self.volumes {
      println!("  {}─{}", branch, metric.filename());
    } else {
      println!("│ {}─{}", branch, metric.filename());
    }
    if self.verbose {
      if last {
        if !self.volumes {
          print!("    └─");
        } else {
          print!("│   └─");
        }
      } else {
        if !self.volumes {
          print!("  │ └─");
        } else {
          print!("│ │ └─");
        }
      }
      if let Some(start_time) = metric.start_time() {
        print!(" start time: {}s", start_time)
      } else {
        print!(" start time: {}", self.no());
      }
      if let Some(duration) = metric.duration() {
        print!(" duration: {}s", duration);
      } else {
        print!(" duration: {}", self.no());
      }
      if let Some(average_duration) = metric.average_duration() {
        print!(" average duration: {}s", average_duration);
      } else {
        print!(" average duration: {}", self.no());
      }
      if let Some(mft_entry_index) = metric.mft_entry_index() {
        print!(" MFT entry index: {}", mft_entry_index);
      } else {
        print!(" MFT entry index: {}", self.no());
      }
      if !last {
        if !self.volumes {
          print!("\n  │\n");
        } else {
          print!("\n│ │\n");
        }
      } else {
        if !self.volumes {
          print!("\n");
        } else {
          print!("\n│");
        }
      }
    }
  }

  fn print_volume(&self, branch: &str, volume:
    &libprefetch::volume::VolumeEntry, last: bool) {
    println!("  {}─{}", branch, volume.device_path());
    if self.verbose {
      if last {
        print!("    ├─");
      } else {
        print!("  │ ├─");
      }
      print!("Creation time: {}", self.format_date(volume.creation_time()));
      println!(" Serial: 0x{:X}", volume.serial_number());
      if last {
        println!("    └─Directories:");
      } else {
        println!("  │ └─Directories:");
      }
      let directories: std::vec::Vec<&str> = volume.directories().unwrap().collect();
      for i in 0 .. directories.len() - 1 {
        if last {
          print!("       ├─");
        } else {
          print!("  │    ├─");
        }
        println!("{} ", directories[i]);
      }
      if last {
        print!("       └─");
      } else {
        print!("  │    └─");
      }
      println!("{} ", directories.last().unwrap());
    }
  }

  fn no<'u>(&self) -> std::borrow::Cow<'u, str> {
    if self.color {
      std::borrow::Cow::Owned(ansi_term::Colour::Red.bold().paint("✘").to_string())
    } else {
      std::borrow::Cow::Borrowed("✘")
    }
  }
}

