# launchd
A Rust library for creating and parsing Launchd files.

## Example

``` rust
use std::path::PathBuf;
use launchd::{CalendarInterval, Error, Launchd};
fn main() -> Result<(), Error> {
    let ci = CalendarInterval::default()
        .with_hour(12)?
        .with_minute(10)?
        .with_weekday(7)?;

    let launchd = Launchd::new("My_Label".to_string(), PathBuf::from("./foo/bar.txt"))
            .with_user_name("Henk".to_string())
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

## Usage
Add this to your Cargo.toml dependencies:
``` toml
launchd = "0.3.0"
```

### Translate crontabs
This is not yet complete.
``` toml
launchd = {version = "0.3.0", features=["cron"]}
```

## Disclaimers
It's still in development and all help is welcome.

The syntax for building launchd files is might still be subject for change. 
Launchd has a lot of properties, and I'm considering using [derive_builder](https://docs.rs/derive_builder/0.9.0/derive_builder/) or something similar to reduce boilerplate code, and increase maintainability. I'm hoping to have made that decision by `v0.4.0`. Suggestions are [welcome](https://github.com/koenichiwa/launchd/issues/4)!

Validity on the `Launchd` struct now remains a responsibility for the user. This could be changed in a future version by using a dedicated builder.

The `Launchd` struct is complete, yet not thorougly tested in production. Especially the code related to the `cron` feature remains untested.

## FAQ
### What is Launchd?
Launchd is MacOS's way of scheduling programs and services to be ran.

For more information: [Wiki](https://en.wikipedia.org/wiki/Launchd).

For a more detailed description of the parameters run `man launchd.plist` on your Apple computer or check out: [manpagez](https://www.manpagez.com/man/5/launchd.plist/) or [xcode-man-pages](https://keith.github.io/xcode-man-pages/launchd.plist.5.html).

### Why not cron?
MacOS deprecated cron, the previous way of scheduling programs.

This library also provides a way of parsing crontabs to `CalendarInterval`s when the `cron` feature is selected.
**Disclaimer**: this feature is still untested.

### Why not systemd?
Due to licensing issues MacOS does not support systemd. 
The parsing of systemd is not included in this library.