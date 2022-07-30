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

use std::collections::HashMap;

/// A struct representing a parsed line.
#[derive(Debug, Clone, Default)]
pub struct Line {
    /// This line's tags. This will be an empty hashmap if there are
    /// none.
    pub tags: HashMap<String, String>,
    /// This line's source (including the nick, user, and host). This is
    /// optional, and will be [`None`] if not provided.
    pub source: Option<String>,
    /// This line's command.
    pub command: String,
    /// Any parameters passed to the command. This will be an empty
    /// vector if there are none.
    pub params: Vec<String>,
}

impl Line {
    /// Creates a new [`Line`]. You should never call this directly, but
    /// instead use the [ircparser::parse](super::parse) function.
    ///
    /// # Arguments
    /// - `tags` - This line's tags.
    /// = `source` - This line's source, or [`None`] if not to be
    /// provided.
    /// - `command` - This line's command.
    /// - `params` - Any parameters passed to the command.
    ///
    /// # Returns
    /// - [`Line`] - The new [`Line`] instance.
    ///
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut tags: HashMap<String, String> = HashMap::new();
    /// tags.insert("id".to_string(), "123".to_string());
    ///
    /// let source = Some(":nick!user@host.tmi.twitch.tv".to_string());
    /// let command = "PRIVMSG";
    /// let params = vec!["#rickastley".to_string()];
    ///
    /// let line = ircparser::Line::new(tags, source, command, params);
    ///
    /// assert_eq!(&line.tags["id"], "123");
    /// assert_eq!(line.source.unwrap(), ":nick!user@host.tmi.twitch.tv");
    /// assert_eq!(line.command, "PRIVMSG");
    /// assert_eq!(line.params[0], "#rickastley");
    /// ```
    pub fn new(
        tags: HashMap<String, String>,
        source: Option<String>,
        command: &str,
        params: Vec<String>,
    ) -> Self {
        Self {
            tags,
            source,
            command: command.to_string(),
            params,
        }
    }
}
