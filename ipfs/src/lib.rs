use bytes::Bytes;
use reqwest::multipart;
use reqwest::multipart::Part;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use url::Url;

mod metadata;

#[derive(Clone, Debug)]
struct Adapter {
    base_url: Url,
    key: String,
    secret: String,
}

impl Adapter {
    pub fn new() -> Self {
        let base_url = env::var("IPFS_URL").expect("IPFS_URL must be set");
        let key = env::var("IPFS_KEY").expect("IPFS_KEY must be set");
        let secret = env::var("IPFS_SECRET").expect("IPFS_SECRET must be set");

        Adapter {
            base_url: base_url.parse().unwrap(),
            key,
            secret,
        }
    }

    pub async fn upload(&self, byte: Bytes, name: String) -> IpfsResult<IpfsOutput> {
        let form = multipart::Form::new().part("file", Part::bytes(byte.to_vec()).file_name(name));

        let mut url = self.base_url.to_owned();
        url.set_path("/api/v0/add");

        let resp = reqwest::Client::new()
            .post(url.to_string())
            .multipart(form)
            .basic_auth(&self.key, Some(&self.secret))
            .send()
            .await?
            .error_for_status()?
            .json::<IpfsOutput>()
            .await
            .map_err(Error::from)?;

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct IpfsOutput {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Hash")]
    pub hash: String,
}

pub async fn create_metadata_from_url(
    name: String,
    description: String,
    image_url: String,
) -> IpfsResult<()> {
    let ipfs = Adapter::new();

    if name.is_empty() || description.is_empty() {
        return Err(Error::Internal("parameter is invalid".to_string()));
    }
    if image_url.is_empty() {
        return Err(Error::Internal("parameter is invalid".to_string()));
    }

    let metadata = metadata::Metadata::new(&name, &image_url, &description);
    let metadata = serde_json::to_string(&metadata).map_err(Error::from)?;
    let content_hash = ipfs.upload(Bytes::from(metadata), name.clone()).await?;
    println!(
        "metadata url: {:?}",
        format!("ipfs://{}", content_hash.hash.clone())
    );
    Ok(())
}

pub async fn create_metadata_from_file(
    name: String,
    description: String,
    image_filename: String,
) -> IpfsResult<()> {
    let ipfs = Adapter::new();

    if name.is_empty() || description.is_empty() {
        return Err(Error::Internal("parameter is invalid".to_string()));
    }
    if image_filename.is_empty() {
        return Err(Error::Internal("parameter is invalid".to_string()));
    }

    let mut file = File::open(format!("asset/{}", image_filename))?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let content_hash = ipfs.upload(Bytes::from(buf), name.clone()).await?;
    let metadata = metadata::Metadata::new(
        &name,
        &format!("ipfs://{}", content_hash.hash.clone()),
        &description,
    );
    let metadata = serde_json::to_string(&metadata).map_err(Error::from)?;
    let content_hash = ipfs.upload(Bytes::from(metadata), name.clone()).await?;
    println!(
        "metadata url: {:?}",
        format!("ipfs://{}", content_hash.hash.clone())
    );

    Ok(())
}

pub type IpfsResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialOrd, PartialEq, Clone)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        let msg = format!("json parse error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        let msg = format!("io error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        let code = e.status().unwrap_or_default();
        let msg = format!("http error: {:?}, code: {:?}", e, code);
        Self::Internal(msg)
    }
}
