use std::fs::File;
use std::io;
use std::io::{BufRead, BufWriter, Write};

use crate::modules::error::Result;

pub fn save_file(path: &str, data: Vec<String>) -> Result<()> {
    let mut buffer = BufWriter::new(File::create(path)?);

    for line in data {
        buffer.write_all(line.as_bytes())?;
    }

    buffer.flush()?;
    Ok(())
}

#[derive(Debug)]
pub struct FileData {
    pub full_name: String,
    pub job_title: String,
    pub department: String,
    pub director: String,
}

pub async fn read_new_data(filename: &str) -> Result<Vec<FileData>> {
    let mut lines = io::BufReader::new(File::open(filename)?).lines();

    let mut file_data: Vec<FileData> = vec![];

    // Пропуск заголовков
    let headers = lines.next();
    if let Some(header) = headers {
        let str = header?;
        let arr: Vec<&str> = str.split(';').collect();
        assert_eq!(arr.len(), 4, "Неверный формат файла");
    }

    for data in lines {
        let str = data?;
        let arr: Vec<&str> = str.split(';').collect();
        let row = FileData {
            full_name: String::from(arr[0].trim()),
            job_title: String::from(arr[1].trim()),
            department: String::from(arr[2].trim()),
            director: String::from(arr[3].trim()),
        };
        file_data.push(row);
    }

    Ok(file_data)
}
