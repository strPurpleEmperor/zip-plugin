#![deny(clippy::all)]

mod tools;
mod zip_file;

use std::fs;
use tools::run;
use zip_file::zip_file;
use crate::tools::{cp_dir};

#[macro_use]
extern crate napi_derive;
#[napi]
fn zip_plugin(dir: String, val: String){
    let dir = cp_dir(&*dir, "./offline");
    run(&*dir, &*val);
    zip_file(&*dir,"offline.zip");
    fs::remove_dir_all(&*dir).unwrap();
}
