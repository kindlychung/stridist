extern crate float_cmp;
extern crate csv;
extern crate rustc_serialize;

#[derive(RustcDecodable, Debug, Copy, Clone)]
pub enum Strategy {Euclidean, Ads}

pub mod distfuncs;
pub mod distcsv;


#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub fn test_dir() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/test");
    d
}

#[cfg(test)]
pub fn test_file(paths: &[&str]) -> PathBuf {
    let mut d = test_dir(); 
    for p in paths {
        d.push(p);
    }
    d
}
