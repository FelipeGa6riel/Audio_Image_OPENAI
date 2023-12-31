use async_openai::{
    types::{CreateImageRequestArgs, ImageSize, ResponseFormat, CreateTranscriptionRequestArgs},
    Client,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let audio_request = CreateTranscriptionRequestArgs::default()
        .file("")
        .model("whisper-1")
        .build()?;
    
    let reponse_mensage = client.audio().transcribe(audio_request).await?;
    println!("{}",reponse_mensage.text);

    let request = CreateImageRequestArgs::default()
        .prompt(reponse_mensage.text)
        .n(4)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()?;

    let response = client.images().create(request).await?;

    // Download and save images to ./data directory.
    // Each url is downloaded and saved in dedicated Tokio task.
    // Directory is created if it doesn't exist.
    let paths = response.save("./data").await?;

    paths
        .iter()
        .for_each(|path| println!("Image file path: {}", path.display()));

    Ok(())
}
