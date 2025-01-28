use std::{fs::File, io::Write};
use zip::write::SimpleFileOptions;
use anyhow::Result;
use glob::glob;
use crate::binaryfile::BinaryReader;


fn compress(zip_filename:&str, target_path:&str) -> Result<()> {
    let mut targets:Vec<String> = Vec::new();
    for entry in glob(target_path).unwrap() {
        match entry {
            Ok(path) => targets.push(path.to_str().unwrap().to_owned()),
            Err(e) => eprint!("{}", e),
        }
    }
    let mut zip = zip::ZipWriter::new(File::create(zip_filename)?);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for target in targets.iter() {
        zip.start_file(target, options)?;
        let br = BinaryReader::open(target)?.read();
        let _ = zip.write(br.unwrap().as_slice());
    }
    zip.finish()?;
    Ok(())
}

#[test]
fn ziptest() {
    compress("test.zip", "./**/*.rs").unwrap();
}
