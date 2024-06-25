use std::error::Error;

use scraper::Html;
use serde_json::{json, Value};

use crate::AccountCredentials;

#[derive(Debug, Clone)]
/// Client for https://mail.tm/
pub struct Client {
    pub client: reqwest::Client,
    pub account_id: String,
    token: String,
    pub email_address: String,
}

impl Client {
    pub async fn new(account_credentials: &AccountCredentials) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();

        let domains = Self::get_domains().await?;
        let domain = &domains[0];
        let address = format!("{}@{}", account_credentials.username, domain);

        let email_address = client
            .post("https://api.mail.tm/accounts")
            .header("Content-Type", "application/json")
            .body(
                json!({"address": address, "password": account_credentials.password }).to_string(),
            )
            .send()
            .await?
            .json::<Value>()
            .await?["address"]
            .as_str()
            .unwrap()
            .to_string();

        let auth_response = client
            .post("https://api.mail.tm/token")
            .header("Content-Type", "application/json")
            .body(
                json!({"address": email_address, "password": account_credentials.password })
                    .to_string(),
            )
            .send()
            .await?
            .json::<Value>()
            .await?;

        let (token, account_id) = (
            auth_response["token"].as_str().unwrap().to_string(),
            auth_response["id"].as_str().unwrap().to_string(),
        );

        Ok(Self {
            client,
            account_id,
            token,
            email_address,
        })
    }

    pub async fn request_latest_message_html(&self) -> Result<Option<Html>, Box<dyn Error>> {
        let response = self
            .client
            .get("https://api.mail.tm/messages")
            .bearer_auth(&self.token)
            .send()
            .await?;

        if let Some(content_length) = response.content_length() {
            if content_length == 0 {
                return Ok(None);
            }
        }

        let messages: Vec<String> = response
            .json::<Value>()
            .await?
            .get("hydra:member")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.get("id").unwrap().as_str().unwrap().to_string())
            .collect();

        if messages.is_empty() {
            return Ok(None);
        }

        let message_id = messages.first().unwrap();

        let response = self
            .client
            .get(format!("https://api.mail.tm/messages/{}", message_id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        if let Some(content_length) = response.content_length() {
            if content_length == 0 {
                return Ok(None);
            }
        }

        let html_string = response
            .json::<Value>()
            .await?
            .get("html")
            .unwrap()
            .as_array()
            .unwrap()
            .first()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        self.client
            .delete(format!("https://api.mail.tm/messages/{}", message_id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        let html = Html::parse_document(&html_string);

        return Ok(Some(html));
    }

    pub async fn delete(&self) -> Result<(), Box<dyn Error>> {
        self.client
            .delete(format!("https://api.mail.tm/token/{}", self.account_id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        Ok(())
    }

    pub async fn get_domains() -> Result<Vec<String>, Box<dyn Error>> {
        Ok(reqwest::get("https://api.mail.tm/domains")
            .await?
            .json::<Value>()
            .await?["hydra:member"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["domain"].as_str().unwrap().to_string())
            .collect())
    }
}
