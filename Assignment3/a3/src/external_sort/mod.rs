use std::fs;
use std::io::Read;

pub fn test() -> std::io::Result<()> {
    fs::create_dir("./temp_sort_files")?;
    Ok(())
}

pub fn find_file() {
//    let mut f = fs::File::open_buffered("./rand_nums.txt")?;
}