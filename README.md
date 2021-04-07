# prefetchkit

[![Crates.io](https://img.shields.io/crates/v/prefetchkit.svg)](https://crates.io/crates/prefetchkit)
[![Crates.io](https://img.shields.io/crates/d/prefetchkit.svg)](https://crates.io/crates/prefetchkit)
[![license](http://img.shields.io/badge/license-WTFPL-blue.svg)](https://github.com/zadlg/prefetchkit/blob/master/LICENSE)


A powerful forensic commandline tool for analyzing and extracting information from
Microsoft Prefetch files.

It fully supports the following Prefetch version:

  * Windows XP/2003
  * Windows Vista/7
  * Windows 8/8.1

If partially supports the following Prefetch version:

  * Windows 10

## Description

`prefetchkit` is a commandline tool which parses and reads Microsoft Prefetch
files.

Prefetch files (with the `.pf` or `.PF` extension) are Windows system files
located in `C:\WINDOWS\Prefetch\`. They help Windows loading executable faster.

`prefetchkit` **is a forensic tool**: it extracts information such as the last
executable which was run, how many times that executable was run.

With the `metrics` option, you can see what files are loaded during the loading
or the executable. For example, if a user launches Paint on a specific picture,
the path to that picture will be stored inside the Prefetch file.

`prefetchkit` uses the [`libprefetch`](https://crates.io/crates/libprefetch)
library for parsing and reading Prefetch files.

## Installation

Using `cargo`:

```bash
cargo intall prefetchkit
```

## Features

`--help`:
```bash
Prefetchkit 1.0.0
A powerful command-line tool for analysing Microsoft Prefetch Files

USAGE:
    prefetchkit [FLAGS] [OPTIONS] <TARGET>

FLAGS:
        --color      Put some colors, it never hurts
    -h, --help       Prints help information
    -m, --metrics    Print metrics (loaded DLL etc)
    -r, --reverse    Reverse order
        --version    Prints version information
    -v, --verbose    Display more information
    -V, --volumes    Print volumes

OPTIONS:
        --sort <sort>    Specify sort (if TARGET is a directory)
                               EXEC: by execution counter
                               TIME: by last execution time
                               NAME: by name [possible values: EXEC, TIME, NAME]

ARGS:
    <TARGET>    Target to analyze (pf files or directory containing pf files)
```

`prefetchkit` takes one positional argument, which can be a specific
Prefetch file or a directory containing Prefetch files.

#### Example
```bash
$ prefetchkit MSPAINT.EXE-11CBB631.pf    # a specific file
$ prefetchkit xpmount/WINDOWS/Prefetch/  # The Windows Prefetch directory
```

### Basics

If you run `prefetchkit` without flags and option, you'll get a table with the
name of the executable, the last execution time and the execution counter:
```bash
$ prefetchkit MSPAINT.EXE-11CBB631.pf
Executable name             Last execution time   Execution counter
MSPAINT.EXE                 2011-03-22 21:44:39                   2
```

On a directory, you can use the `--sort=<VALUE>` option for sorting by:

  * `EXEC` - execution counter
  * `TIME` - last execution time
  * `NAME` - name

Additionally, there is `-r` for reversing the sort.

### Metrics

Metrics is a special section of the Prefetch file which indicates each DLL,
DAT (and other) files which are loaded with the executable. Depending on the
Windows version, you get additional information such as the average loading time.

To display metrics, use the flag `-m`:
```bash
$ prefetchkit UPDATE.EXE-0CB058D8.pf -m
Executable name             Last execution time   Execution counter
UPDATE.EXE                  2011-03-13 11:09:24                   2
└─Metrics:
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\NTDLL.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\KERNEL32.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\UNICODE.NLS
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\LOCALE.NLS
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\SORTTBLS.NLS
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SOFTWAREDISTRIBUTION\DOWNLOAD\38F47E51C38A7A0EBC9C39DCA1EDD5A6\UPDATE\UPDATE.EXE
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\ADVAPI32.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\RPCRT4.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\COMCTL32.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\GDI32.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\USER32.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\CRYPT32.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\MSVCRT.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\MSASN1.DLL
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\IMAGEHLP.DLL
....
```
Using the verbose flag (`-v`), you'll get the additional information:
```bash
$ prefetechkit UPDATE.EXE-0CB058D8.pf -mv
Executable name             Last execution time   Execution counter
UPDATE.EXE                  2011-03-13 11:09:24                   2
└─Metrics:
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\NTDLL.DLL
  │ └─ start time: 0s duration: 50s average duration: ✘ MFT entry index: ✘
  │
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\KERNEL32.DLL
  │ └─ start time: 50s duration: 52s average duration: ✘ MFT entry index: ✘
  │
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\UNICODE.NLS
  │ └─ start time: 102s duration: 5s average duration: ✘ MFT entry index: ✘
  │
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\LOCALE.NLS
  │ └─ start time: 107s duration: 3s average duration: ✘ MFT entry index: ✘
  │
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\SORTTBLS.NLS
  │ └─ start time: 110s duration: 4s average duration: ✘ MFT entry index: ✘
  │
  ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SOFTWAREDISTRIBUTION\DOWNLOAD\38F47E51C38A7A0EBC9C39DCA1EDD5A6\UPDATE\UPDATE.EXE
  │ └─ start time: 114s duration: 57s average duration: ✘ MFT entry index: ✘
....
```

### Volumes

When you launch a executable, it uses files on one or several volumes. This
kind of information is stored inside the Prefetch file. Even if the executable is
stored on a external volume, a Prefetch file will be created.

In a forensic point a view, it can be very useful: you can determine that a USB key
has been used with Paint, Chrome or another software, or a special software is located
on a external hard drive.

For volumes, use the volume flag: `-V`:
```bash
$ prefetchkit UPDATE.EXE-0CB058D8.pf -V
Executable name             Last execution time   Execution counter
UPDATE.EXE                  2011-03-13 11:09:24                   2
└─Volumes:
  └─\DEVICE\HARDDISKVOLUME1
```

Again, using the verbose flag `-v`, you'll get extra information, such as each
directory which is used by the executable:
```bash
$ prefetchkit UPDATE.EXE-0CB058D8.pf -Vv
Executable name             Last execution time   Execution counter
UPDATE.EXE                  2011-03-13 11:09:24                   2
└─Volumes:
  └─\DEVICE\HARDDISKVOLUME1
    ├─Creation time: 2009-03-04 10:23:57 Serial: 0x1054BA98
    └─Directories:
       ├─\DEVICE\HARDDISKVOLUME1\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\INF\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SOFTWAREDISTRIBUTION\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SOFTWAREDISTRIBUTION\DOWNLOAD\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SOFTWAREDISTRIBUTION\DOWNLOAD\38F47E51C38A7A0EBC9C39DCA1EDD5A6\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SOFTWAREDISTRIBUTION\DOWNLOAD\38F47E51C38A7A0EBC9C39DCA1EDD5A6\UPDATE\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\CATROOT\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\SYSTEM32\CATROOT\{F750E6C3-38EE-11D1-85E5-00C04FC295EE}\
       ├─\DEVICE\HARDDISKVOLUME1\WINDOWS\WINSXS\
       └─\DEVICE\HARDDISKVOLUME1\WINDOWS\WINSXS\X86_MICROSOFT.WINDOWS.COMMON-CONTROLS_6595B64144CCF1DF_6.0.2600.2180_X-WW_A84F1FF9\
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## Compatibility

`ole` seems to work for rust 1.9 and greater.

## License

<http://www.wtfpl.net/about/>
