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

extern crate libprefetch;
extern crate clap;
extern crate ansi_term;
extern crate chrono;

mod kit;
mod formatter;

fn main() {
  let matches = clap::App::new("Prefetchkit")
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(clap::Arg::with_name("TARGET")
      .help("Target to analyze (pf files or directory containing pf files)")
      .required(true)
      .index(1)
    )
    .arg(clap::Arg::with_name("sort")
      .long("sort")
      .help("Specify sort (if TARGET is a directory)
      EXEC: by execution counter
      TIME: by last execution time
      NAME: by name")
      .takes_value(true)
      .required(false)
      .possible_value("EXEC")
      .possible_value("TIME")
      .possible_value("NAME")
    )
    .arg(clap::Arg::with_name("reverse")
      .long("reverse")
      .short("r")
      .help("Reverse order")
      .required(false)
    )
    .arg(clap::Arg::with_name("color")
      .long("color")
      .help("Put some colors, it never hurts")
      .required(false)
    )
    .arg(clap::Arg::with_name("metrics")
      .long("metrics")
      .short("m")
      .help("Print metrics (loaded DLL etc)")
      .required(false)
    )
    .arg(clap::Arg::with_name("verbose")
      .long("verbose")
      .short("v")
      .help("Display more information")
      .required(false)
    )
    .arg(clap::Arg::with_name("volumes")
      .long("volumes")
      .short("V")
      .help("Print volumes")
    )
  .get_matches();

  let target = matches.value_of("TARGET").unwrap();
  let path = std::path::Path::new(target);
  if path.exists() {
    if path.is_file() {
      match kit::file(&matches, &path) {
        Err(e) => eprintln!("{}", e),
        _ => {}
      }
    } else {
      match kit::dir(&matches, &path) {
        Err(e) => eprintln!("{}", e),
        _ => {}
      }
    }
  } else {
    eprintln!("{:?}: no such file or directory", path);
  }
}
