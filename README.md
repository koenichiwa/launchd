# launchd
A Rust library for creating and parsing Launchd files.

It's still in early development and all help is welcome.

## FAQ
### What is Launchd?
Launchd is MacOS's way of scheduling programs and services to be ran.

For more information: [Wiki](https://en.wikipedia.org/wiki/Launchd).

For a more detailed description of the parameters run `man launchd.plist` on your Apple computer or check out: [manpagez](https://www.manpagez.com/man/5/launchd.plist/).

### Why not cron?
MacOS deprecated cron, the previous way of scheduling programs.

This library also provides a way of parsing crontabs to CalendarIntervals when the `cron` feature is selected.
**Disclaimer**: this feature is still untested.

### Why not systemd?
Due to licensing issues MacOS does not support systemd. 
The parsing of systemd is not included in this library.

## Usage

Add this to your Cargo.toml dependencies:
``` toml
launchd = "0.1.0"
```

## Features
### Default
``` toml
launchd = {version = "0.1.0", features=["io"]}
```
### Translate crontabs
``` toml
launchd = {version = "0.1.0", features=["cron"]}
```
### Without the plist writer
``` toml
launchd = {version = "0.1.0", default-features = false, features=["serde"]}
```


## Example

``` rust
use std::path::Path;
use launchd::{CalendarInterval, Error, Launchd};
fn main() -> Result<(), Error> {
    let ci = CalendarInterval::default()
        .with_hour(12)?
        .with_minute(10)?
        .with_weekday(7)?;

    let launchd = Launchd::new("LABEL", Path::new("./foo/bar.txt"))?
            .with_user_name("Henk")
            .with_program_arguments(vec!["Hello".to_string(), "World!".to_string()])
            .with_start_calendar_intervals(vec![ci])
            .disabled();
    
    launchd.to_writer_xml(std::io::stdout())
}
```

Results in:

``` xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
        <key>Label</key>
        <string>LABEL</string>
        <key>Disabled</key>
        <true />
        <key>UserName</key>
        <string>Henk</string>
        <key>Program</key>
        <string>./foo/bar.txt</string>
        <key>ProgramArguments</key>
        <array>
                <string>Hello</string>
                <string>World!</string>
        </array>
        <key>StartCalendarIntervals</key>
        <array>
                <dict>
                        <key>Minute</key>
                        <integer>10</integer>
                        <key>Hour</key>
                        <integer>12</integer>
                        <key>Weekday</key>
                        <integer>7</integer>
                </dict>
        </array>
</dict>
</plist>
```
