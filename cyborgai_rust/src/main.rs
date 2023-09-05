use std::io::Write;
use std::path::Path;
use std::error::Error;
use reqwest::{Client, multipart};
use sanitize_filename::sanitize;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpServer, post, get, HttpResponse, http, Responder};
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use mime_guess::MimeGuess;
use mime::Mime;
use futures::StreamExt;

static BASE_URL: Lazy<String> = Lazy::new(|| "http://127.0.0.1:5001".to_string());
static BASE_URL_GW: Lazy<String> = Lazy::new(|| "http://127.0.0.1:8080".to_string());
#[derive(Serialize, Deserialize)]
pub struct UploadResponse {
    cid: String,
    status: u8,
    error: String,
}

pub struct CIpfs {
    client: Client,
    base_url: String,
  
}

impl CIpfs {
    pub fn new(base_url: &str) -> CIpfs {
        CIpfs {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn do_upload(&self, file_path: &str) -> Result<String, Box<dyn Error>> {
        let path = Path::new(file_path);
        let file_name = path.file_name().ok_or("Invalid file path")?.to_string_lossy().to_string();

        let mut file = File::open(file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let part = multipart::Part::bytes(buffer).file_name(file_name.clone());

        let form = multipart::Form::new().part("file", part);

        let res = self.client
            .post(&format!("{}/api/v0/add", self.base_url))
            .multipart(form)
            .send()
            .await?;

        let text = res.text().await?;
        let json: serde_json::Value = serde_json::from_str(&text)?;
        let hash = json.get("Hash").ok_or("Missing 'Hash' field in response")?.as_str().unwrap().to_string();

        println!("Upload response: {:?}", text);

        Ok(hash)
    }

    pub async fn do_publish(&self, ipfs_hash: &str) -> Result<(), Box<dyn Error>> {
        let res = self.client
            .post(&format!("{}/api/v0/name/publish", self.base_url))
            .query(&[("arg", ipfs_hash)])
            .send()
            .await?;

        let text = res.text().await?;
        println!("Publish response: {:?}", text);

        Ok(())
    }

    pub async fn do_download(&self, ipfs_hash: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
        let res = self.client
            .get(&format!("{}/api/v0/cat?arg={}", self.base_url, ipfs_hash))
            .send()
            .await?;
        let content = res.bytes().await?;

        let mut file = File::create(file_path).await?;
        file.write_all(&content).await?;

        Ok(())
    }
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    let ipfs = CIpfs::new(&BASE_URL);
    let mut cid = String::new();
    let mut status: u8 = 0;
    let mut error = String::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        let content_type = field.content_disposition();
        let filename = match content_type.get_filename() {
            Some(filename) => filename,
            None => continue,
        };
        let filepath = format!("./tmp/{}", sanitize(&filename));

        // Create the directory if it doesn't exist
        fs::create_dir_all("./tmp").await.unwrap();

        let mut f = File::create(filepath.clone()).await.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await.unwrap();
        }

        match ipfs.do_upload(&filepath).await {
            Ok(cid_value) => {
                cid = cid_value;
                ipfs.do_publish(&cid).await.unwrap(); 
            }
            Err(e) => {
                status = 1;
                error = e.to_string();
                break;
            }
        }
    }

    let response = UploadResponse {
        cid,
        status,
        error,
    };

    HttpResponse::Ok().json(&response)
}

#[get("/download/{cid}")]
async fn download(cid: web::Path<String>) -> impl Responder {
    let ipfs = CIpfs::new(&BASE_URL);

    let filepath = format!("./download/{}", &cid);

    // Create the directory if it doesn't exist
    fs::create_dir_all("./download").await.unwrap();

    ipfs.do_download(&cid, &filepath.clone()).await.unwrap();

    format!("Downloaded file with CID: {}", &cid)
}

#[get("/preview/{cid}/{ext}")]
async fn preview(info: web::Path<(String, String)>) -> actix_web::Result<HttpResponse> {
    let ipfs = CIpfs::new(&BASE_URL);
    let cid = &info.0;
    let ext = &info.1;

    let res = ipfs.client
        .get(&format!("{}/api/v0/cat?arg={}", BASE_URL_GW.to_string(), cid))
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

   /*  if !res.status().is_success() {
        return Err(actix_web::error::ErrorInternalServerError("Failed to get data from IPFS"));
    }*/

    let content = res.bytes().await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if content.is_empty() {
        return Err(actix_web::error::ErrorInternalServerError("No data found on IPFS"));
    }
     
    
    let mime_type = guess_mime_type(ext);

    Ok(HttpResponse::Ok()
        .content_type(mime_type.to_string())
        .body(content))
}


fn guess_mime_type(ext: &str) -> Mime {
    let mime_type = MimeGuess::from_ext(ext).first_or_octet_stream();
    mime_type
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: i32 = 8081;
    let address: &str = "0.0.0.0";

    println!("Server Start: {}:{:?}", address, port);
    HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(download)
            .service(preview)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}