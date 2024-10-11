use reqwest::{Response, StatusCode};
use serde::Deserialize;
use tracing::{debug, info};

use crate::modules::config::Config;
use crate::modules::error::{Error, Result};

#[derive(Deserialize)]
struct ErrorResponse {
    #[serde(rename(deserialize = "errorMessage"))]
    error_message: String,
}
pub struct Worker {
    base_url: String,
    app_id: String,
    secret_key: String,
}

impl Worker {
    pub fn new(is_demo: bool) -> Self {
        let config = Config::new();
        let base_url = if is_demo {
            config.base_url_demo
        } else {
            config.base_url
        };

        Self {
            base_url,
            app_id: config.app_id,
            secret_key: config.secret_key,
        }
    }

    fn build_url(&self, url: &str) -> String {
        let new_url = format!(
            "{}{url}&appid={}&secretkey={}",
            self.base_url, self.app_id, self.secret_key
        );
        format!(
            "{}{url}&appid={}&sign={}",
            self.base_url,
            self.app_id,
            sign(&new_url)
        )
    }

    pub async fn fetch_all_persons(&self, offset: i32) -> Result<Response> {
        let url = self.build_url(&format!(
            "persons?bean_add_fields=directorid&limit=200&offset={offset}"
        ));
        info!("Получаем записи начиная с: {}", offset);
        debug!("[fetch_all_persons] url: {url}");
        let client = reqwest::Client::new();
        let res = client.get(url).send().await?;
        Ok(res)
    }

    pub async fn patch_person_director(
        &self,
        person_id: &str,
        director_id: &str,
    ) -> Result<Response> {
        let url = self.build_url(&format!("persons/{person_id}?directorid={director_id}"));
        debug!("[patch_person_director] url: {url}");
        call_put(url).await
    }

    pub async fn patch_person_job(&self, person_id: &str, job_title: &str) -> Result<Response> {
        let url = self.build_url(&format!("persons/{person_id}?rspostidname={job_title}"));
        debug!("[patch_person_job] url: {url}");
        call_put(url).await
    }
}

fn sign(url: &str) -> String {
    format!("{:x}", md5::compute(url))
}

async fn call_put(url: String) -> Result<Response> {
    let client = reqwest::Client::new();
    let res = client.put(url).send().await?;
    let status = res.status();
    if status != StatusCode::OK {
        let response = res.json::<ErrorResponse>().await?;
        return Err(Error::Fetch(response.error_message));
    }
    Ok(res)
}
