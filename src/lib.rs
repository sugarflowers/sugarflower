pub mod path;
pub mod env;

use crate::path::Path;
use crate::env::get_env;

#[test]
fn str_path() {
    let mut p = Path::new();
    p.set("a\\b\\c");
    println!("{:?}", p.get());
    println!("{:?}", p.parent());
}

#[test]
fn get_env_test() {
    println!("{:?}", get_env("USERPROFILE"));
    println!("{:?}", get_env("INVALIDKEY"));
}
