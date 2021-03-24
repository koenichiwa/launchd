# launchd
A Rust library for creating and parsing Launchd files.
It's still in early development all help is welcome.

## Example

``` rust
use std::path::Path;
use launchd::{CalendarInterval, Error, Launchd}
fn main() -> Result<(), Error> {
    let ci = CalendarInterval::new()
        .with_hour(12)?
        .with_minute(10)?
        .with_weekday(7)?;

    let launchd = Launchd::new("LABEL".to_string(), Path::new("./foo/bar.txt"))?
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
