static SECRET: &'static str = "";

use copypasta::{ClipboardContext, ClipboardProvider};
use data_encoding::{BASE32, BASE64};
use oath::{totp_raw_custom_time, HashType};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let secret = get_secret();
    let rhex: Vec<u8> = if secret.contains("=") {
        BASE64.decode(secret.as_bytes()).unwrap()
    } else {
        BASE32.decode(secret.as_bytes()).unwrap()
    };
    let current_time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards")
        .as_secs();
    let left_time = 30 - current_time.wrapping_rem(30);
    let rstring: u64 = totp_raw_custom_time(&rhex, 10, 0, 30, current_time, &HashType::SHA1);
    let passcode: String = base26(&rstring);
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(passcode.to_owned()).unwrap();
    println!("{}\n{}s left", passcode, left_time);
    // Extra time for writting xcb or it would fail
    std::thread::sleep(Duration::from_millis(10));
}

#[cfg(feature = "predefined_secret")]
fn get_secret() -> String {
    SECRET.to_string()
}

#[cfg(not(feature = "predefined_secret"))]
fn get_secret() -> String {
    if SECRET.len() > 0 {
        return SECRET.to_string();
    }
    match std::env::var("SECRET") {
        Ok(result) => return result.to_string(),
        Err(err) => {
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                return args[1].clone().to_string();
            } else {
                eprintln!(
                    "Please pass args for secret or set `SECRET` environment variable!\n{}",
                    err
                );
                std::process::exit(1);
            };
        }
    };
}

fn base26(num: &u64) -> String {
    let mut encode = num.clone();
    let mut decode = String::new();
    let chars: Vec<char> = "23456789BCDFGHJKMNPQRTVWXY".chars().collect();
    for _ in 0..5 {
        let pchar = chars[(encode as usize).wrapping_rem(chars.len())];
        decode.push(pchar);
        encode = encode.wrapping_div(chars.len() as u64);
    }
    decode
}
