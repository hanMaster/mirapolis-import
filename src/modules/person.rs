use std::slice::Iter;

use serde::Deserialize;

use crate::modules::error::Result;
use crate::modules::fetch::Worker;
use crate::modules::files::{read_new_data, save_record};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Person {
    pub personid: String,
    pub plastname: String,
    pub pfirstname: String,
    pub psurname: String,
    isuser: bool,
    pilogin: String,
    pipassword: String,
    caid: String,
    pub caidname: String,
    rspostid: String,
    pub rspostidname: String,
    ppsex: String,
    personemail: String,
    pstatus: String,
    pextcode: String,
    pub directorid: String,
    pub directoridname: String,
}

async fn get_list(is_demo: bool) -> Result<Vec<Person>> {
    let mut full_list: Vec<Person> = vec![];
    let mut offset = 0;
    let worker = Worker::new(is_demo);
    let res = worker.fetch_all_persons(offset).await?;
    let mut list = res.json::<Vec<Person>>().await?;
    while list.len() == 200 {
        full_list.append(&mut list);
        offset += 200;
        let res = worker.fetch_all_persons(offset).await?;
        list = res.json::<Vec<Person>>().await?;
    }
    full_list.append(&mut list);
    Ok(full_list)
}

fn find_director<'a>(fio: &str, mut iter: Iter<'a, Person>) -> Option<&'a Person> {
    iter.find(|p| {
        let full_name = format!("{} {} {}", p.plastname, p.pfirstname, p.psurname);
        full_name.eq(&fio)
    })
}

pub async fn update_person_director(filename: &str, is_demo: bool) -> Result<()> {
    let list = get_list(is_demo).await?;
    println!("Получено записей: {}", list.len());

    let file_data = read_new_data(filename).await?;
    for file in file_data {
        let saved = list
            .iter()
            .find(|p| format!("{} {} {}", p.plastname, p.pfirstname, p.psurname).eq(&file.fio));
        if let Some(site) = saved {
            if site.directoridname.ne(&file.director) {
                println!(
                    "Изменить для: {}; было: {}; будет: {}",
                    file.fio, site.directoridname, file.director
                );
                let dir_option = find_director(&file.director, list.iter());
                if let Some(director) = dir_option {
                    println!("Отправлен запрос на изменение данных");
                    let worker = Worker::new(is_demo);
                    let res = worker
                        .patch_person_director(&site.personid, &director.personid)
                        .await;
                    match res {
                        Ok(_) => {
                            println!("Успешно");
                        }
                        Err(err) => {
                            println!("{:?}", err);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn update_person_job(filename: &str, is_demo: bool) -> Result<()> {
    let list = get_list(is_demo).await?;
    println!("Получено записей: {}", list.len());

    let file_data = read_new_data(filename).await?;
    for file in file_data {
        let saved = list
            .iter()
            .find(|p| format!("{} {} {}", p.plastname, p.pfirstname, p.psurname).eq(&file.fio));
        if let Some(site) = saved {
            if site.rspostidname.ne(&file.job_title) {
                println!(
                    "Изменить для: {}; было: {}; будет: {}",
                    file.fio, site.rspostidname, file.job_title
                );

                println!("Отправлен запрос на изменение данных");
                let worker = Worker::new(is_demo);
                let res = worker
                    .patch_person_job(&site.personid, &file.job_title)
                    .await;
                match res {
                    Ok(_) => {
                        println!("Успешно");
                    }
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn save_list(is_demo: bool) -> Result<()> {
    let list = get_list(is_demo).await?;
    println!("Получено записей: {}", list.len());
    save_record("Id;ФИО;Должность;Подразделение;Руководитель")?;
    for p in list {
        let record = format!(
            "{};{} {} {};{};{};{}",
            p.personid,
            p.plastname,
            p.pfirstname,
            p.psurname,
            p.rspostidname,
            p.caidname,
            p.directoridname
        );
        save_record(&record)?;
    }
    Ok(())
}
