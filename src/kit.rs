// This file is part of prefetchkit.
//
// prefetchkit is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// prefetchkit is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with prefetchkit.  If not, see <http://www.gnu.org/licenses/>.
//
// Authors: zadig <thomas chr(0x40) bailleux.me>
//          jasa <jan.starke (0x40) t-systems.com>

use std;
use clap;
use libprefetch;

// directory
pub(crate) fn dir(matches: &clap::ArgMatches, path: &std::path::Path)
    -> Result<(), Box<dyn std::error::Error>> {
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
    -> Result<(), Box<dyn std::error::Error>> {
  let mut f = super::formatter::Formatter::new();
  f.set_options(matches);
  if let Some(p) = path.to_str() {
    let prefetch = libprefetch::Prefetch::from_path(p)?;
    f.print_prefix();
    f.print(&prefetch);
  }
  Ok(())
}
