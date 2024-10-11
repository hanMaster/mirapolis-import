use serde::Deserialize;
use tracing::{debug, error, info};

use crate::modules::error::Result;
use crate::modules::fetch::Worker;
use crate::modules::files::{read_new_data, save_file};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Person {
    #[serde(rename(deserialize = "personid"))]
    person_id: String,
    #[serde(rename(deserialize = "plastname"))]
    last_name: String,
    #[serde(rename(deserialize = "pfirstname"))]
    first_name: String,
    #[serde(rename(deserialize = "psurname"))]
    sur_name: String,
    #[serde(rename(deserialize = "isuser"))]
    is_user: bool,
    #[serde(rename(deserialize = "pilogin"))]
    login: String,
    #[serde(rename(deserialize = "pipassword"))]
    password: String,
    #[serde(rename(deserialize = "caid"))]
    department_id: String,
    #[serde(rename(deserialize = "caidname"))]
    department_name: String,
    #[serde(rename(deserialize = "rspostid"))]
    job_id: String,
    #[serde(rename(deserialize = "rspostidname"))]
    job_title: String,
    #[serde(rename(deserialize = "ppsex"))]
    sex: String,
    #[serde(rename(deserialize = "personemail"))]
    person_email: String,
    #[serde(rename(deserialize = "pstatus"))]
    person_status: String,
    #[serde(rename(deserialize = "pextcode"))]
    ext_code: String,
    #[serde(rename(deserialize = "directorid"))]
    director_id: String,
    #[serde(rename(deserialize = "directoridname"))]
    director_name: String,
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

fn find_director<'a>(name: &str, mut iter: impl Iterator<Item = &'a Person>) -> Option<&'a Person> {
    iter.find(|p| {
        let full_name = format!("{} {} {}", p.last_name, p.first_name, p.sur_name);
        full_name.eq(&name)
    })
}

pub async fn update_person_director(filename: &str, is_demo: bool) -> Result<()> {
    let list = get_list(is_demo).await?;
    info!("Получено записей: {}", list.len());

    let file_data = read_new_data(filename).await?;
    for file in file_data {
        let saved = list.iter().find(|p| {
            format!("{} {} {}", p.last_name, p.first_name, p.sur_name).eq(&file.full_name)
        });

        if let Some(site) = saved {
            if site.director_name.ne(&file.director) {
                info!(
                    "Изменить для: {}; было: {}; будет: {}",
                    file.full_name, site.director_name, file.director
                );
                let dir_option = find_director(&file.director, list.iter());
                debug!("{:?}", &dir_option);
                if let Some(director) = dir_option {
                    info!("Отправлен запрос на изменение данных");
                    let worker = Worker::new(is_demo);
                    let res = worker
                        .patch_person_director(&site.person_id, &director.person_id)
                        .await;
                    match res {
                        Ok(_) => {
                            info!("Успешно");
                        }
                        Err(err) => {
                            error!("{:?}", err);
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
    info!("Получено записей: {}", list.len());

    let file_data = read_new_data(filename).await?;
    for file in file_data {
        let saved = list.iter().find(|p| {
            format!("{} {} {}", p.last_name, p.first_name, p.sur_name).eq(&file.full_name)
        });
        if let Some(site) = saved {
            if site.job_title.ne(&file.job_title) {
                info!(
                    "Изменить для: {}; было: {}; будет: {}",
                    file.full_name, site.job_title, file.job_title
                );

                info!("Отправлен запрос на изменение данных");
                let worker = Worker::new(is_demo);
                let res = worker
                    .patch_person_job(&site.person_id, &file.job_title)
                    .await;
                match res {
                    Ok(_) => {
                        info!("Успешно");
                    }
                    Err(err) => {
                        error!("{:?}", err);
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn save_list(path: &str, is_demo: bool) -> Result<()> {
    let list = get_list(is_demo).await?;
    info!("Получено записей: {}", list.len());
    let mut data_for_save: Vec<String> = Vec::with_capacity(list.len() + 1);
    data_for_save.push("Id;ФИО;Должность;Подразделение;Руководитель".to_string());

    for p in list {
        let record = format!(
            "{};{} {} {};{};{};{}\n",
            p.person_id,
            p.last_name,
            p.first_name,
            p.sur_name,
            p.job_title,
            p.department_name,
            p.director_name
        );
        data_for_save.push(record);
    }
    save_file(path, data_for_save)?;
    Ok(())
}
