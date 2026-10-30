use std::fs;

#[allow(unused)]
pub fn read_files() -> std::io::Result<()> {
    let path = "./";
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}
