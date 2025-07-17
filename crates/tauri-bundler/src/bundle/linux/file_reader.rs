use xpath_reader::reader::Reader;
use std::fs::File;
use std::io::BufReader;

pub fn read_file_content(file_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(&file_path)?;
    let buf_reader = BufReader::new(file);
    let mut reader = Reader::from_reader(buf_reader);
    
    //SINK
    let content = reader.read(&file_path)?;
    Ok(content)
}

pub async fn process_file_read(target_path: String) -> Result<String, Box<dyn std::error::Error>> {
    read_file_content(target_path)
} 