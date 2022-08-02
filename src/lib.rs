// BSD 3-Clause License
//
// Copyright (c) 2022-present, Ethan Henderson
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this
//    list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! An IRC (RFC1459) parser and formatter, built in Rust.
//!
//! ## Parsing messages
//!
//! You can parse IRC messages using the provided `parse` function.
//!
//! ```
//! let msg = "@id=123;name=rick :nick!user@host.tmi.twitch.tv PRIVMSG #rickastley :Never gonna give you up!";
//! match ircparser::parse(msg) {
//!     Ok(mut x) => {
//!         let line = x.pop_front().unwrap();
//!
//!         assert_eq!(&line.tags["id"], "123");
//!         if line.source.is_some() {
//!             assert_eq!(line.source.unwrap(), ":nick!user@host.tmi.twitch.tv");
//!         }
//!         assert_eq!(line.command, "PRIVMSG");
//!         assert_eq!(line.params[0], "#rickastley");
//!         assert_eq!(line.params[1], "Never gonna give you up!");
//!     }
//!     Err(e) => {
//!         println!("A parsing error occured: {e}");
//!         return;
//!     }
//! };
//! ```

mod line;

pub use line::Line;
use std::collections::{HashMap, VecDeque};

type ParseResult<T> = Result<T, ParseError>;

/// Exception thrown when an error occurs during message parsing.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The details of this error.
    pub details: String,
}

impl ParseError {
    /// Generates a new [`ParseError`].
    ///
    /// # Arguments
    /// - `details` - THe details of this error.
    ///
    /// # Example
    /// ```
    /// let e = ircparser::ParseError::new("err");
    ///
    /// assert_eq!(e.details, "err".to_string())
    /// ```
    pub fn new(details: &str) -> Self {
        Self {
            details: details.into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

fn find_index(text: &str, char: char, start: usize) -> Option<usize> {
    for (k, _) in text.match_indices(char) {
        if k > start {
            return Some(k);
        }
    }

    None
}

/// Parses an IRC message.
///
/// # Arguments
/// - `text` - The text you want to parse. This can comprise of multiple
/// lines. In this case, each line (separated by a newline character)
/// will be a separate element in the return value.
///
/// # Returns
/// - [`VecDeque<Line>`] - A [`VecDeque`] of all parsed [`Line`]s.
///
/// # Example
/// ```
/// let msg = "@id=123;name=rick :nick!user@host.tmi.twitch.tv PRIVMSG #rickastley :Never gonna give you up!";
///
/// match ircparser::parse(msg) {
///     Ok(mut x) => {
///         let line = x.pop_front().unwrap();
///
///         assert_eq!(&line.tags["id"], "123");
///         if line.source.is_some() {
///             assert_eq!(line.source.unwrap(), ":nick!user@host.tmi.twitch.tv");
///         }
///         assert_eq!(line.command, "PRIVMSG");
///         assert_eq!(line.params[0], "#rickastley");
///         assert_eq!(line.params[1], "Never gonna give you up!");
///     }
///     Err(e) => {
///         println!("A parsing error occured: {e}");
///         return;
///     }
/// };
/// ```
///
/// # Notice
/// The behaviour of this function changed in v0.2.0. It can now accept
/// multiple lines at once, but as a consequence, now returns a
/// [`VecDeque`] of [`Line`] objects instead of a single [`Line`].
pub fn parse(text: &str) -> ParseResult<VecDeque<Line>> {
    let mut parsed_lines: VecDeque<Line> = VecDeque::new();

    for line in text.replace('\r', "").split('\n') {
        if line.is_empty() {
            return Err(ParseError::new("line length cannot be 0"));
        }

        let mut idx = 0;
        let mut tags: HashMap<String, String> = HashMap::new();
        let mut source: Option<String> = None;

        // Parse tags component.
        if line.starts_with('@') {
            idx = line.find(' ').unwrap();

            for part in Some(&line[1..idx]).unwrap().split(';') {
                let kv: Vec<&str> = part.split('=').collect();
                tags.insert(kv[0].to_string(), kv[1].to_string());
            }

            idx += 1;
        }

        // Parse source component.
        if line.chars().nth(idx).unwrap() == ':' {
            let end_idx = find_index(line, ' ', idx).unwrap();
            source = Some(line[idx..end_idx].to_string());
            idx = end_idx + 1;
        }

        // Parse command component.
        let end_idx = find_index(line, ' ', idx).unwrap();
        let command = &line[idx..end_idx];
        idx = end_idx + 1;

        let c_idx = match find_index(line, ':', idx) {
            Some(x) => x - 1,
            None => line.len(),
        };

        // Parse params component.
        let mut params: Vec<String> = line[idx..c_idx].split(' ').map(|x| x.to_string()).collect();
        if c_idx != line.len() {
            params.push(line[c_idx + 2..].to_string());
        }

        parsed_lines.push_back(Line::new(tags, source, command, params));
    }

    Ok(parsed_lines)
}

#[cfg(test)]
mod test_lib {
    use super::parse;
    use collection_macros::hashmap;
    use std::collections::HashMap;

    #[test]
    fn test_single_partial() {
        let msg = "PRIVMSG #rickastley :Never gonna give you up!";
        match parse(msg) {
            Ok(mut x) => {
                let line = x.pop_front().unwrap();

                assert_eq!(line.tags, HashMap::new());
                assert_eq!(line.source, None);
                assert_eq!(line.command, "PRIVMSG");
                assert_eq!(line.params, vec!["#rickastley", "Never gonna give you up!"]);
            }
            Err(e) => {
                println!("A parsing error occured: {e}");
                return;
            }
        }
    }

    #[test]
    fn test_single_full() {
        let msg = "@id=123;name=rick :nick!user@host.tmi.twitch.tv PRIVMSG #rickastley :Never gonna give you up!";
        match parse(msg) {
            Ok(mut x) => {
                let line = x.pop_front().unwrap();

                assert_eq!(
                    line.tags,
                    hashmap! {
                        String::from("id") => String::from("123"),
                        String::from("name") => String::from("rick"),
                    }
                );
                assert_eq!(
                    line.source,
                    Some(String::from(":nick!user@host.tmi.twitch.tv"))
                );
                assert_eq!(line.command, "PRIVMSG");
                assert_eq!(line.params, vec!["#rickastley", "Never gonna give you up!"]);
            }
            Err(e) => {
                println!("A parsing error occured: {e}");
                return;
            }
        };
    }

    #[test]
    fn test_readme_example() {
        let msg = "@id=123;name=rick :nick!user@host.tmi.twitch.tv PRIVMSG #rickastley :Never gonna give you up!";
        match parse(msg) {
            Ok(mut x) => {
                let line = x.pop_front().unwrap();

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

    #[test]
    fn test_multiline() {
        let msg = "@id=123 PRIVMSG #rickastley :Never gonna give you up!\n@id=456 PRIVMSG #rickastley :Never gonna let you down!";
        match parse(msg) {
            Ok(mut x) => {
                assert_eq!(x.len(), 2);

                let l1 = x.pop_front().unwrap();
                let l2 = x.pop_front().unwrap();

                assert_eq!(&l1.tags["id"], "123");
                assert_eq!(&l2.tags["id"], "456");
                assert_eq!(l1.command, l2.command);
                assert_eq!(l1.params[1], "Never gonna give you up!");
                assert_eq!(l2.params[1], "Never gonna let you down!");
            }
            Err(e) => {
                println!("A parsing error occured: {e}");
                return;
            }
        }
    }
}
