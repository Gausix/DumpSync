use reqwest::Client;

use std::{
    error::Error,
    collections::HashMap,
};

use crate::{
    utils::file::FileUtils,
    constants::global::Global,
    ui::share_alerts::ShareAlerts,
};

pub struct Share {
    file: String,
    api_key: String,
}

impl Share {

    pub fn new(file: &str, api_key: &str) -> Self {
        Self {
            file: file.to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn share(&self) -> Result<(), Box<dyn Error>> {
        let ext = FileUtils::extension(&self.file);

        if ["sql", "txt", "csv", "json", "html"].iter().any(|&e| e == ext) {
            let privacy = "1".to_string();
            let api_option = "paste".to_string();
            let name = format!("{}: {}", Global::APP_NAME, &self.file);
            let content = FileUtils::content(&self.file);
            
            let mut params = HashMap::new();
            params.insert("api_dev_key", &self.api_key);
            params.insert("api_option", &api_option);
            params.insert("api_paste_code", &content);
            params.insert("api_paste_private", &privacy);
            params.insert("api_paste_name", &name);
            params.insert("api_paste_format", &ext);
            
            let response = Client::new()
            .post(Global::PASTEBIN_API_URI)
            .form(&params)
            .send()
            .await?;
            
            let response_text = response.text().await?;
            if response_text.starts_with("http") {
                ShareAlerts::success(&response_text);
            } else {
                ShareAlerts::error(&response_text);
            }
        } else {
            ShareAlerts::error("Invalid file extension");
        }

        Ok(())
    }

}