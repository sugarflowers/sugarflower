pub mod path;
pub mod env;
pub mod sjis;

use crate::path::Path;
use crate::env::get_env;
use crate::sjis::*;


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
