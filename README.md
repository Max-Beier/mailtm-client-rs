# MailTM Client

[<img alt="crates.io" src="https://img.shields.io/crates/v/mailtm-client" height="20" />](https://crates.io/crates/mailtm-client)
[<img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/Max-Beier/mailtm-client-rs/rust.yml" height="20" />](https://github.com/Max-Beier/mailtm-client-rs/actions)

An asynchronous and simple client for [Mail.tm](https://mail.tm/de/).

## Getting started

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_credentials =
        mailtm_client::AccountCredentials::new("User123", "my_secret_password");

    let email_client = mailtm_client::Client::new(&account_credentials).await?;

    let mail;
    loop {
        if let Some(html) = email_client.request_latest_message_html().await? {
            mail = html;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
```
