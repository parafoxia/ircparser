# ircparser

An IRC (RFC1459) parser and formatter, built in Rust.

`ircparser` should work on basically any Rust version, but the earliest version checked in the CI is v1.31 (the first version with Rust 2018 support).

## Setup

To use the latest stable version of `ircparser`, add it to your Cargo.toml file like so:

```toml
[dependencies]
ircparser = "^0.2.1"
```

You can also use the latest development version by specifying the following:

```toml
[dependencies]
ircparser = { git = "https://github.com/parafoxia/ircparser" }
```

## Usage

`ircparser` currently only has one public function — `parse`.
This function takes a line of an IRC message, and parses it into an easy-to-use `Line` object.

```rs
use ircparser;

fn main() {
    let msg = "@id=123;name=rick :nick!user@host.tmi.twitch.tv PRIVMSG #rickastley :Never gonna give you up!";
    match ircparser::parse(msg) {
        Ok(x) => {
            let line = x;

            assert_eq!(&line.tags["id"], "123");
            if line.source.is_some() {
                assert_eq!(line.source.unwrap(), ":nick!user@host.tmi.twitch.tv");
            }
            assert_eq!(line.command, "PRIVMSG");
            assert_eq!(line.params[0], "#rickastley");
            assert_eq!(line.params[1], "Never gonna give you up!");
        }
        Err(e) => {
            println!("A parsing error occured: {e}");
            return;
        }
    };
}
```

## License

The `ircparser` crate for Rust is licensed under the [BSD 3-Clause License](https://github.com/parafoxia/ircparser/blob/main/LICENSE).
