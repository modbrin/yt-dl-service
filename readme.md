# yt-dl-service
A minimalistic wrapper for `yt-dlp` which is designed to run as a service and keep track of e.g. youtube channels.
If there's a risk that certain channel can be taken down, this utility helps to keep up-to-date backup locally.

This is achieved by running `yt-dlp` in background for a list of links according to schedule.

> *DISCLAIMER: This is a personal project, tailored to my needs. Therefore
> no PKGBUILD or prebuilt binaries are provided just yet. Follow
> [manual installation](#manual-installation) if you want to continue.*

## Manual Installation
1. **Get [yt-dlp](https://github.com/yt-dlp/yt-dlp)**, depending on your platform. For example on Arch:
```sh
$ sudo pacman -S yt-dlp
```
> As a result of installation, `yt-dlp` must be available in PATH - check by running `yt-dlp --version`.

2. **Get rust toolchain** if you don't have it already, usually this is done via [rustup](https://rustup.rs/). For example on Arch:
```sh
$ sudo pacman -S rustup
$ rustup toolchain install stable
```

3. **Build executable**
```sh
$ cd yt-dl-service
$ cargo install --path . --root ~/.cargo
```
> Binary `yt-dl-service` will be located in `~/.cargo/bin/`

4. **Configure service**

4.1 Copy Necessary files
```sh
$ sudo mkdir /etc/yt-dl-service
$ sudo mkdir /var/log/yt-dl-service 
$ sudo cp ./templates/settings.json /etc/yt-dl-service/
$ sudo cp ./templates/yt-dl-service.service /etc/systemd/system/
```
4.2 Change username in service
```sh
$ sudo <YOUR FAVORITE EDITOR> /etc/systemd/system/yt-dl-service.service
```

4.3 Customize settings, refer to [settings chapter](#settings). At this point you want add desired time (UTC tz) in `updateSchedule` and add some channels to `tasks`.
```sh
$ sudo <YOUR FAVORITE EDITOR> /etc/yt-dl-service/settings.json
```
Also be sure to set `"updateOnStart": true` on first launch in order to perform initial download and check if everything is working ok. Don't forget to set it back to `false` once you've ensured it's working.

> Alternatively, you can keep it always on. In that case channels will be updated on boot and when you restart the service. 

4.4 Enable systemd service
```sh
$ sudo systemctl enable --now yt-dl-service.service
$ sudo systemctl status yt-dl-service.service
``` 

5. **Summary**

As a result, you should have:
* binary `/home/username/.cargo/bin/yt-dl-service`
* config `/etc/yt-dl-service/settings.json`
* log `/var/log/yt-dl-service/yt-dl-service.log`
* systemd config `/etc/systemd/system/yt-dl-service.service`

6. **Adding new channels**

Modify settings file and reload
```sh
sudo systemctl restart yt-dl-service.service
```

## Settings
Configuration is done solely via `settings.json` file.
Generally, it should look like this:
```json
{
    "tasks": [
        {
            "url": "<YT CHANNEL URL A>",
            "outputPath": "/home/myusername/channels-backup/channel-a",
            "name": "This is A"
        },
        {
            "url": "<YT CHANNEL URL B>",
            "outputPath": "/home/myusername/channels-backup/channel-b",
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
- FIX: Output files are owned by root because of service
- FIX: Remove tmp dir when empty
- FEAT: Handle blocking queue if schedule ping overlaps
- FEAT: Update settings validation
- FEAT: Detect download failures, attempt failed downloads separately
- FEAT: Make settings cache to store pre-processed values
