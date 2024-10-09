use reqwest::Response;
use serde::Deserialize;

use crate::modules::config::config;
use crate::modules::error::{Error, Result};

#[derive(Deserialize)]
struct ErrorResponse {
    #[serde(rename(deserialize = "errorMessage"))]
    error_message: String,
}
pub struct Worker<'a> {
    base_url: &'a str,
}

impl<'a> Worker<'a> {
    pub fn new(is_demo: bool) -> Self {
        let base_url = if is_demo {
            &config().BASE_URL_DEMO
        } else {
            &config().BASE_URL
        };

        Self { base_url }
    }

    pub async fn fetch_all_persons(&self, offset: i32) -> Result<Response> {
        let url = format!(
            "{}persons?bean_add_fields=directorid&limit=200&offset={offset}&{}",
            self.base_url,
            &config().CREDS
        );

        let encrypted_url = format!(
            "{}persons?bean_add_fields=directorid&limit=200&offset={offset}&appid=dns&sign={}",
            self.base_url,
            sign(&url)
        );

        println!("Получаем записи начиная с: {}", offset);
        let client = reqwest::Client::new();
        let res = client.get(encrypted_url).send().await?;
        Ok(res)
    }

    pub async fn patch_person_director(
        &self,
        person_id: &str,
        director_id: &str,
    ) -> Result<Response> {
        let url = format!(
            "{}persons/{person_id}?directorid={director_id}&{}",
            self.base_url,
            &config().CREDS
        );

        let encrypted_url = format!(
            "{}persons/{person_id}?directorid={director_id}&appid=dns&sign={}",
            self.base_url,
            sign(&url)
        );
        call_put(encrypted_url).await
    }

    pub async fn patch_person_job(&self, person_id: &str, job_title: &str) -> Result<Response> {
        let url = format!(
            "{}persons/{person_id}?rspostidname={job_title}&{}",
            self.base_url,
            &config().CREDS
        );

        let encrypted_url = format!(
            "{}persons/{person_id}?rspostidname={job_title}&appid=dns&sign={}",
            self.base_url,
            sign(&url)
        );
        call_put(encrypted_url).await
    }
}
fn sign(url: &str) -> String {
    format!("{:x}", md5::compute(url))
}

async fn call_put(url: String) -> Result<Response> {
    let client = reqwest::Client::new();
    let res = client.put(url).send().await?;
    let status = res.status();
    if status == 500 {
        let response = res.json::<ErrorResponse>().await?;
        return Err(Error::Fetch(response.error_message));
    }
    Ok(res)
}
