photo-renamer
=============

Rename picture and media files based on their exif data

Example
--------

```
$ photo-renamer -a copy -d ./target-dir ~/Desktop/IMG_8670.HEIC
Copy /Users/dave/Desktop/IMG_8670.HEIC -> ./target-dir/2022/09/IMG_8670.HEIC
```

```
$ file target-dir/2022/09/IMG_8670.HEIC
target-dir/2022/09/IMG_8670.HEIC: ISO Media, HEIF Image HEVC Main or Main Still Picture Profile
$ tree ./target-dir/
./target-dir/
└── 2022
    └── 09
        └── IMG_8670.HEIC

2 directories, 1 file
```

Building
--------

    cargo build
    cargo run

Usage
-----

```
photo-renamer

USAGE:
    photo-renamer [OPTIONS] [FILES]...

ARGS:
    <FILES>...    Photos to process

OPTIONS:
    -a, --action <action>     Action to take for file organization [default: move] [possible values:
                              move, copy, hardlink]
    -d, --target-dir <dir>    Directory to output files to [default: .]
    -h, --help                Print help information
    -n, --dry-run             Don't actually take any action
```

Debugging
---------

`photo-renamer` uses `env_logger`:

    RUST_LOG=trace cargo run -q -- <opts>

```
$ RUST_LOG=trace cargo run -q -- --dry-run ~/Desktop/IMG_8670.HEIC
[2022-09-19T23:09:44Z DEBUG photo_renamer] Args {
        action: Move,
        target_dir: ".",
        dry_run: true,
        files: [
            "/Users/dave/Desktop/IMG_8670.HEIC",
        ],
    }
[2022-09-19T23:09:44Z TRACE photo_renamer] "/Users/dave/Desktop/IMG_8670.HEIC" -> DateTime {
        year: 2022,
        month: 9,
        day: 19,
        hour: 16,
        minute: 13,
        second: 45,
        nanosecond: None,
        offset: None,
    }
[2022-09-19T23:09:44Z DEBUG photo_renamer] "/Users/dave/Desktop/IMG_8670.HEIC" -> "./2022/09/IMG_8670.HEIC"
[dry-run] Move /Users/dave/Desktop/IMG_8670.HEIC -> ./2022/09/IMG_8670.HEIC
```

License
-------

MIT License
