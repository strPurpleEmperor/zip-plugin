use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub fn zip_file(folder_path: &str, file_name: &str) {
  run(folder_path, file_name).unwrap();
}

fn run(folder_path: &str, out_name: &str) -> io::Result<()> {
  // 创建ZIP文件
  let file = File::create(out_name)?;
  let mut zip = ZipWriter::new(BufWriter::new(file));
  let options = SimpleFileOptions::default();

  let base_path = Path::new(folder_path);

  // 遍历文件夹下的所有条目
  let entries = std::fs::read_dir(base_path)?;
  for entry in entries {
    let entry = entry?;
    let entry_path = entry.path();
    // 计算条目相对于文件夹的路径
    let relative_path = entry_path
      .strip_prefix(base_path)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?; // 转换错误类型
    let relative_name = relative_path.to_str().unwrap().replace("\\", "/");

    if entry_path.is_dir() {
      // 添加目录条目（以/结尾）
      let dir_name = format!("{}/", relative_name);
      zip.add_directory(dir_name, options)?;
      // 递归处理子目录
      add_dir_contents(&mut zip, options, base_path, &entry_path)?;
    } else {
      // 添加文件到ZIP
      zip.start_file(relative_name, options)?;
      let mut file = File::open(entry_path)?;
      io::copy(&mut file, &mut zip)?;
    }
  }

  zip.finish()?;
  println!("文件夹压缩成功。");
  Ok(())
}

// 递归处理子目录
fn add_dir_contents(
  zip: &mut ZipWriter<BufWriter<File>>,
  options: SimpleFileOptions,
  base_path: &Path,
  current_dir: &Path,
) -> io::Result<()> {
  for entry in std::fs::read_dir(current_dir)? {
    let entry = entry?;
    let entry_path = entry.path();
    let relative_path = entry_path
      .strip_prefix(base_path)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?; // 转换错误类型
    let relative_name = relative_path.to_str().unwrap().replace("\\", "/");

    if entry_path.is_dir() {
      // 添加目录条目并递归处理
      let dir_name = format!("{}/", relative_name);
      zip.add_directory(dir_name, options)?;
      add_dir_contents(zip, options, base_path, &entry_path)?;
    } else {
      // 添加文件
      zip.start_file(relative_name, options)?;
      let mut file = File::open(entry_path)?;
      io::copy(&mut file, zip)?;
    }
  }
  Ok(())
}
