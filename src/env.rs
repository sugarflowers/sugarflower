use std::env;

pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or("".to_string())
}

#[test]
fn all_var() {
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
#[test]
fn get_test() {
    println!(">>{}", get_env("USERPROFILE"));
    println!(">>{}", get_env("INVALIDKEY"));
}

