#![deny(clippy::all)]

mod tools;
mod zip_file;

use crate::tools::cp_dir;
use std::fs;
use tools::run;
use zip_file::zip_file;

#[macro_use]
extern crate napi_derive;
#[napi]
fn zip_plugin(dir: String, val: Option<String>) {
  let dir = cp_dir(&*dir, "./offline");
  if let Some(val) = val {
    run(&*dir, &*val);
  }
  zip_file(&*dir, "offline.zip");
  fs::remove_dir_all(&*dir).unwrap();
}
