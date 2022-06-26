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
            "url": "<YT CHANNEL LINK A>",
            "outputPath": "~/channels-backup/channel-a",
            "name": "This is A"
        },
        {
            "url": "<YT CHANNEL LINK B>",
            "outputPath": "~/channels-backup/channel-b",
            "name": "This is B"
        },
    ],
    "updateSchedule": "* 5 * * * *",
    "logLevel": "Trace",
    "updateOnStart": true
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
Examples for `maxResolution`: 480, 720, 1080, 1440, 2160, 4320
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
Examples for `throttleSpeed`: 3.5M, 300K
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