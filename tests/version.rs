extern crate curl;

use std::process::{Command, Stdio};

use curl::Version;

#[cfg(any(target_os = "macos"))]
#[test]
fn check_homebrews_curl() {
    let brew = Command::new("brew").arg("list").stdout(Stdio::piped()).spawn();
    let use_homebrew = match brew {
        Ok(_) => {
            let grep = Command::new("grep")
                .args(&["-w", "curl"])
                .stdin(Stdio::piped())
                .output()
                .unwrap();
            match grep.status.code() {
                Some(0) => true,
                _ => false,
            }
        },
        Err(_) => false,
    };
    let v = Version::get();
    if use_homebrew {
        assert!(v.version() != "7.54.0");
        assert_eq!(v.version(), "7.58.0");
    } else {
        // bundled libcurl is 7.54.0 on macos10.12 and macos10.13
        assert!(v.version() == "7.54.0");
    }
}
