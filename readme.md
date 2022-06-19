# yt-dl-service
A simple wrapper for `yt-dlp` which is designed to run as a service and keep track of e.g. youtube channels.
If there's a risk that certain channel can be taken down, this utility helps to keep up-to-date backup locally.

## Build
1. Get [yt-dlp](https://github.com/yt-dlp/yt-dlp), depending on your platform. For example on Arch:
```sh
$ sudo pacman -S yt-dlp
```
> As a result of installation, `yt-dlp` must be available in PATH - check by running `yt-dlp --version`.

2. Get rust toolchain if you don't have it already, usually this is done via [rustup](https://rustup.rs/). For example on Arch:
```sh
$ sudo pacman -S rustup
$ rustup toolchain install stable
```

Build executable
```sh
$ cargo build --release
$ ./target/release/yt-dl-service
```
Configure service
> TODO


## Download format
Currently, no format indication is passed to `yt-dlp` meaning best settings will be used.
This may be undesirable in some scenarios, e.g. when channel provides 4k content but 1080p backup would be enough.
Or if only audio is needed.