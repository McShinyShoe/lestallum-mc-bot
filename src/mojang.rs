use reqwest::{Client, multipart};
use std::{fs, path::PathBuf};

pub async fn change_skin(token: &String, path: &str) -> anyhow::Result<()> {
    let client = Client::new();
    let path = PathBuf::from(path);

    let bytes = fs::read(path)?;
    let part = multipart::Part::bytes(bytes)
        .file_name("skin.png")
        .mime_str("image/png")?;

    let form = multipart::Form::new()
        .text("variant", "classic") // use "slim" for Alex model
        .part("file", part);

    let res = client
        .post("https://api.minecraftservices.com/minecraft/profile/skins")
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await?;
        anyhow::bail!("skin upload failed: {}", text);
    }

    Ok(())
}
