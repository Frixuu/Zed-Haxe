use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;
use serde::Serialize;
use zed_extension_api::{self as zed};

#[derive(Serialize)]
pub enum LibraryVersion {
    Git,
    Haxelib { semver: String },
    Dev { path: String },
}

fn parse_version_string(version_str: &str) -> LibraryVersion {
    if version_str.eq_ignore_ascii_case("git") {
        LibraryVersion::Git
    } else if version_str.starts_with("dev:") {
        LibraryVersion::Dev {
            path: version_str[4..].to_string(),
        }
    } else {
        LibraryVersion::Haxelib {
            semver: version_str.to_string(),
        }
    }
}

pub fn get_installed_libraries() -> Result<HashMap<String, LibraryVersion>, ()> {
    let mut command = zed::Command {
        command: "haxelib".to_string(),
        args: vec!["list".to_string()],
        env: vec![],
    };

    match command.output() {
        Err(_) => Err(()),
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut installed_libraries = HashMap::new();
            stdout.lines().for_each(|line| {
                if let Some(index) = line.find(':') {
                    let name = &line[0..index];
                    static RE: LazyLock<Regex> =
                        LazyLock::new(|| Regex::new(r#"\[(?P<version>[^\]]+)\]"#).unwrap());
                    if let Some(captures) = RE.captures(line) {
                        if let Some(capture_match) = captures.name("version") {
                            installed_libraries.insert(
                                name.to_string(),
                                parse_version_string(capture_match.as_str()),
                            );
                        }
                    }
                }
            });
            Ok(installed_libraries)
        }
    }
}
