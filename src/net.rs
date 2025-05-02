// use std::{
//     fs,
//     io::Write,
//     path::PathBuf,
//     // str::Bytes,
// };

use bytes::Bytes;

// use tokio_stream::StreamExt;

// pub async fn download_file(url: &str, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = fs::File::create(path)?;
//     println!("Downloading {}...", url);

//     let mut stream = reqwest::get(url).await?.bytes_stream();

//     // let mut i: u32 = 0;
//     while let Some(chunk_result) = stream.next().await {
//         let chunk = chunk_result?;
//         Write::write_all(&mut file, &chunk)?;

//         // println!("{}", stream.size_hint().0);
//     }

//     // file.flush().await?;
//     file.flush()?;

//     println!("Downloaded {}", url);
//     Ok(())
// }

pub async fn download_file_bytes(
    url: &str,
    // path: PathBuf,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let bytes = reqwest::get(url).await?.bytes().await?;
    Ok(bytes)
}
