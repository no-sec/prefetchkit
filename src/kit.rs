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
use clap;
use libprefetch;

// directory
pub(crate) fn dir(matches: &clap::ArgMatches, path: &std::path::Path)
    -> Result<(), Box<std::error::Error>> {
  let files = std::fs::read_dir(path)?;

  let mut pfs = std::vec::Vec::<libprefetch::Prefetch>::new();

  for f in files {
    if f.is_ok() {
      if let Some(p) = f.unwrap().path().to_str() {
        let pf = libprefetch::Prefetch::from_path(p);
        match pf {
          Ok(pp) => pfs.push(pp),
          Err(e) => eprintln!("warning: {} {}", p, e)
        }
      }
    }
  }

  // check ord
  if matches.is_present("sort") {
    match matches.value_of("sort").unwrap() {
      "EXEC" => pfs.sort_by(|a, b|
        a.execution_counter().cmp(&b.execution_counter())),
      "TIME" => pfs.sort_by(|a, b|
        a.last_execution_time().cmp(&b.last_execution_time())),
      "NAME" => pfs.sort_by(|a, b|
        a.name().cmp(&b.name())),
      _ => {}
    }
  }

  // reverse ?
  if matches.is_present("reverse") {
    pfs.reverse();
  }
  let mut f = super::formatter::Formatter::new();
  f.set_options(matches);
  f.print_prefix();
  for pf in pfs {
    f.print(&pf);
  }
  Ok(())
}

// file
pub(crate) fn file(matches: &clap::ArgMatches, path: &std::path::Path)
    -> Result<(), Box<std::error::Error>> {
  let mut f = super::formatter::Formatter::new();
  f.set_options(matches);
  if let Some(p) = path.to_str() {
    let prefetch = libprefetch::Prefetch::from_path(p)?;
    f.print_prefix();
    f.print(&prefetch);
  }
  Ok(())
}
