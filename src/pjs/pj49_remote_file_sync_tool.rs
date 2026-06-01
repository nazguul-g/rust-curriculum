use std::fs;
use std::io;
use std::path::{Path};
use std::time::SystemTime;

pub fn remote_file_sync_tool() -> io::Result<()> {
    let src = prompt("enter source path: ");
    let dst = prompt("enter destination path: ");
    sync_dirs(Path::new(&src), Path::new(&dst))?;

    Ok(())
}
fn sync_dirs(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            sync_dirs(&src_path, &dst_path)?;
        } else if is_new_or_updated(&src_path, &dst_path)? {
            fs::copy(&src_path, &dst_path)?;
            println!("{} copied into {}", src_path.display(), dst_path.display())
        }
    }
    Ok(())
}
fn is_new_or_updated(src: &Path, dst: &Path) -> io::Result<bool> {
    if !dst.exists() {
        return Ok(true);
    }
    let src_metadata = src.metadata()?;
    let dst_metadata = dst.metadata()?;
    let src_modified = src_metadata
        .modified()
        .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
    let dst_modified = dst_metadata
        .modified()
        .unwrap_or_else(|_| SystemTime::UNIX_EPOCH);
    Ok(src_modified > dst_modified)
}
fn prompt(msg: &str) -> String {
    use std::io::{Write, stdin, stdout};
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
