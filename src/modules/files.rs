use std::io::{BufRead, Write};
use std::{fs, io};

use crate::modules::error::Result;

pub fn save_record(info: &str) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("list.csv")?;

    let data = format!("{}\n", info);

    file.write_all(data.as_bytes())?;
    Ok(())
}

#[derive(Debug)]
pub struct Payload {
    pub fio: String,
    pub job_title: String,
    pub department: String,
    pub director: String,
}

pub async fn read_new_data(filename: &str) -> Result<Vec<Payload>> {
    let file = fs::File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut payload: Vec<Payload> = vec![];

    // Пропуск заголовков
    let headers = lines.next();
    if let Some(header) = headers {
        let str = header?;
        let arr: Vec<&str> = str.split(';').collect();
        assert_eq!(arr.len(), 4, "Неверный вормат файла");
    }

    for data in lines {
        let str = data?;
        let arr: Vec<&str> = str.split(';').collect();
        let row = Payload {
            fio: String::from(arr[0].trim()),
            job_title: String::from(arr[1].trim()),
            department: String::from(arr[2].trim()),
            director: String::from(arr[3].trim()),
        };
        payload.push(row);
    }

    Ok(payload)
}
