# MailTM Client

An asynchronous and simple client for [Mail.tm](https://mail.tm/de/).

## Getting started

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let account_credentials =
        mailtm_client::AccountCredentials::new("User123", "my_secret_password");

    let email_client = mailtm_client::Client::new(&account_credentials).await?;

    let mail;
    loop {
        if let Some(html) = email_client.request_latest_message_html().await? {
            mail = html;
            break;
        }
    }

    Ok(())
}
```
