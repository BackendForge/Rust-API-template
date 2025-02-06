// use std::env;
// use std::fs;
// use std::path::Path;

pub fn main() {
    //     let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    //     println!("cargo:info=env_path={:?}", env_path);
    //     let env = fs::read_to_string(env_path).expect("Failed to read .env file at CARGO_MANIFEST_DIR");
    //     for line in env.lines() {
    //         // skip if starts with #
    //         if line.starts_with("#") {
    //             continue;
    //         }
    //         let mut parts = line.split("=");
    //         let key = parts.next().expect("Key not found");
    //         let value = parts.next().expect("Value not found");
    //         println!("cargo::rustc-env={}={}", key, value);
    //         env::set_var(key, value);
    //     }
    //     // std::env::var("DB_HOST").expect("DB_HOST not set");
    //     // std::env::var("DB_USERNAME").expect("DB_USERNAME not set");
    //     // std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    //     // std::env::var("DB_NAME").expect("DB_NAME not set");

    //     println!(
    //         "cargo::rustc-env=CARGO_MANIFEST_DIR={}",
    //         env!("CARGO_MANIFEST_DIR")
    //     );
}
