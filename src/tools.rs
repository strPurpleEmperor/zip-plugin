use regex::Regex;
use std::fs::DirEntry;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::{fs, io};

pub fn run(dir: &str, link_to_replace: &str) {
  for entry in walk_dir(dir) {
    if let Ok(entry) = entry {
      let path = entry.path();
      if path.is_file() && path.extension().unwrap() == "html" {
        replace_link(&path, link_to_replace);
      }
    }
  }
}

pub fn walk_dir(dir: &str) -> Vec<Result<DirEntry, Error>> {
  fs::read_dir(dir)
    .unwrap()
    .map(|entry| entry.map(|entry| entry))
    .into_iter()
    .collect::<Vec<_>>()
}

fn replace_link(path: &Path, link_to_replace: &str) {
  let file = fs::File::open(path).unwrap();
  let reader = BufReader::new(file);

  let mut content = String::new();

  for line in reader.lines() {
    let line = line.unwrap();
    let re = Regex::new(link_to_replace).unwrap();
    let new_line = re.replace_all(&*line, "");
    content.push_str(&new_line);
    content.push('\n');
  }

  fs::write(path, content).unwrap();
}

pub fn cp_dir(old_path: &str, new_path: &str) -> String {
  copy_dir(old_path.as_ref(), new_path.as_ref()).unwrap();
  new_path.to_string()
}
fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
  if !src.is_dir() {
    return Err(Error::new(
      io::ErrorKind::InvalidInput,
      "Source is not a directory",
    ));
  }

  // 创建目标目录
  fs::create_dir_all(dest)?;

  // 遍历源目录中的内容
  for entry in fs::read_dir(src)? {
    let entry = entry?;
    let entry_path = entry.path();
    let file_name = entry.file_name();

    // 构建目标路径
    let dest_path = dest.join(&file_name);

    if entry_path.is_dir() {
      // 如果是子目录，则递归地调用 copy_dir
      copy_dir(&entry_path, &dest_path)?;
    } else {
      // 如果是文件，则直接复制
      fs::copy(&entry_path, &dest_path)?;
    }
  }

  Ok(())
}
