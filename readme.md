# yt-dl-service
A minimalistic wrapper for `yt-dlp` which is designed to run as a service and keep track of e.g. youtube channels.
If there's a risk that certain channel can be taken down, this utility helps to keep up-to-date backup locally.

This is achieved by running `yt-dlp` in background for a list of links according to schedule.

> *DISCLAIMER: This is a personal project, tailored to my needs. Therefore
> no PKGBUILD or prebuilt binaries are provided. Follow
> [manual installation](#manual-installation) if you want to continue.*

## Manual Installation
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


## Settings
Configuration is done solely via `settings.json` file.
Generally, it should look like this:
```json
{
    "tasks": [
        {
            "url": "<YT CHANNEL URL A>",
            "outputPath": "~/channels-backup/channel-a",
            "name": "This is A"
        },
        {
            "url": "<YT CHANNEL URL B>",
            "outputPath": "~/channels-backup/channel-b",
            "name": "This is B"
        },
    ],
    "updateSchedule": [{"daily":"20:30"}],
    "updateOnStart": true
}
```

### Example, make schedule:
**Note that all time is in UTC, not your local time.**
Global setting, all tasks are executed at specified time(s).
Can be either `daily` with single time or `cron` with custom time.
In example below, it will execute daily at 12:00 and 18:30 UTC, also at 8:00 on mondays of year 2049.
If you are not familiar with `cron`, refer [here](https://en.wikipedia.org/wiki/Cron) and directly to [crate](https://crates.io/crates/tokio-cron-scheduler) docs.
```json
{
    "tasks": [
        ...
    ],
    "updateSchedule": [
        { "daily": "12:00" },
        { "daily": "18:30" },
        { "cron": "0 0 8 * * 1 2049" }
    ],
    ...
}
```

### Example, download only audio:
Task-specific setting, if `audioFormat` field is not present, will default to mp3.
Possible values for `audioFormat` are: mp3, aac, m4a, opus, vorbis, flac, alac, wav
```json
"tasks": [
    {
        ...
        "audioOnly": true,
        "audioFormat": "wav"
    },
    ...
],
```

### Example, limit video resolution:
Task-specific setting.
Examples for `maxResolution`: 480, 720, 1080, 1440, 2160, 4320 ...
```json
"tasks": [
    {
        ...
        "maxResolution": 720
    },
    ...
],
```


### Example, limit download speed:
Task-specific setting.
Examples for `throttleSpeed`: 3.5M, 300K ...
```json
"tasks": [
    {
        ...
        "throttleSpeed": "500K"
    },
    ...
],
```

### Example, custom yt-dlp flags:
Task-specific setting.
Pass any flags supported by yt-dlp to achieve desired behavior.
```json
"tasks": [
    {
        ...
        "customFlags": ["-f", "bestvideo[height=480]+bestaudio"]
    },
    ...
],
```


## TODO
- Handle blocking queue if schedule ping overlaps
- Update settings validation
- Remove tmp dir when empty
- Detect download failures, attempt failed downloads separately
- Make settings cache to store pre-processed values