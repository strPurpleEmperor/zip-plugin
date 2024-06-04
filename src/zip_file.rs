use std::fs::File;
use std::io::{self, prelude::*, BufWriter};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
pub fn zip_file(folder_path:&str, file_name: &str) {
    run(folder_path,file_name).unwrap()
}
fn run(folder_path:&str, out_name: &str) -> io::Result<()> {
    // 创建一个压缩文件
    let file = File::create(out_name)?;
    let mut zip = ZipWriter::new(BufWriter::new(file));

    // 定义压缩选项
    let options = SimpleFileOptions::default();

    // 压缩文件夹中的所有文件
    let files = std::fs::read_dir(folder_path)?;
    for file in files {
        let file = file?;
        let file_path = file.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // 如果是文件夹则跳过
        if file_path.is_dir() {
            continue;
        }
        // 添加文件到压缩文件中，并将文件路径更改为"offline"
        let offline_path = format!("/{}", file_name);
        zip.start_file(folder_path.to_owned() + &*offline_path, options)?;
        let mut file_reader = File::open(file_path)?;
        io::copy(&mut file_reader, &mut zip)?;
    }

    // 完成压缩文件
    zip.finish()?;

    println!("Folder compressed successfully.");
    Ok(())
}
