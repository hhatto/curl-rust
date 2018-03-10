extern crate curl;

use std::process::{Command, Stdio};

use curl::Version;

#[cfg(any(target_os = "macos"))]
#[test]
fn check_homebrews_curl() {
    let brew = Command::new("brew").arg("list").stdout(Stdio::piped()).spawn();
    let use_homebrew = match brew {
        Ok(mut b) => {
            let mut grep = Command::new("grep")
                .args(&["-w", "curl"])
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            {
                let out = b.stdout.as_mut().unwrap();
                let in_ = grep.stdin.as_mut().unwrap();
                std::io::copy(out, in_).unwrap();
            }
            let _brew_status = b.wait().unwrap();
            let grep_status = grep.wait().unwrap();
            match grep_status.code() {
                Some(0) => true,
                _ => false,
            }
        },
        Err(e) => {
            println!("fail brew command: {:?}", e);
            false
        },
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
