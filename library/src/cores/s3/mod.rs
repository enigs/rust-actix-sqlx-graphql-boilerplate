pub mod form;
pub mod queries;

use arraygen::Arraygen;
use async_graphql::{Result, SimpleObject};
use image::GenericImageView;
use infer::Infer;
use rusoto_core::credential::{ StaticProvider };
use rusoto_core::{ HttpClient, Region };
use rusoto_s3::{ PutObjectRequest, S3 as RS3, S3Client };
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File as StdFile;
use std::io::Read;
use std::str::FromStr;

use crate::Asset;
use crate::Errors;

pub use form::{S3Form, S3Error};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[derive(macros::SetCipher, macros::SetIsEmpty, macros::SetMutate, sqlx::Type)]
#[derive(SimpleObject)]
#[gen_array(fn get_ciphers: &mut String)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "JSONB")]
pub struct S3 {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub access_key_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub secret_access_key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub bucket: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[in_array(get_ciphers)]
    pub region: String,
    pub image_thumbnail_small_size: i32,
    pub image_thumbnail_medium_size: i32,
    pub image_thumbnail_large_size: i32,
    pub image_thumbnail_xl_size: i32,
    pub image_landscape_width_small_size: i32,
    pub image_landscape_height_small_size: i32,
    pub image_landscape_width_medium_size: i32,
    pub image_landscape_height_medium_size: i32,
    pub image_landscape_width_large_size: i32,
    pub image_landscape_height_large_size: i32,
    pub image_landscape_width_xl_size: i32,
    pub image_landscape_height_xl_size: i32,
    pub image_landscape_width_xxl_size: i32,
    pub image_landscape_height_xxl_size: i32,
    pub image_landscape_width_xxxl_size: i32,
    pub image_landscape_height_xxxl_size: i32,
}

impl S3 {
    pub fn get_thumbnail_sizes(&self) ->  Vec<(u32, u32)> {
        vec![
            (self.image_thumbnail_xl_size as u32, self.image_thumbnail_xl_size as u32),
            (self.image_thumbnail_large_size as u32, self.image_thumbnail_large_size as u32),
            (self.image_thumbnail_medium_size as u32, self.image_thumbnail_medium_size as u32),
            (self.image_thumbnail_small_size as u32, self.image_thumbnail_small_size as u32),
        ]
    }

    pub fn get_landscape_sizes(&self) ->  Vec<(u32, u32)> {
        vec![
            (self.image_landscape_width_xxxl_size as u32, self.image_landscape_height_xxxl_size as u32),
            (self.image_landscape_width_xxl_size as u32, self.image_landscape_height_xxl_size as u32),
            (self.image_landscape_width_xl_size as u32, self.image_landscape_height_xl_size as u32),
            (self.image_landscape_width_large_size as u32, self.image_landscape_height_large_size as u32),
            (self.image_landscape_width_medium_size as u32, self.image_landscape_height_medium_size as u32),
            (self.image_landscape_width_small_size as u32, self.image_landscape_height_small_size as u32),
        ]
    }

    pub fn get_client(&self) -> Result<S3Client> {
        if self.is_empty() {
            return Err(Errors::internal_server_error("S3 is not initialized..."));
        }

        // Set aws keys and region
        let access_key = self.access_key_id.clone();
        let secret_access_key = self.secret_access_key.clone();
        let region = Region::from_str(&self.region)
            .map_err(Errors::internal_server_error)?;

        // Set aws credentials
        let credentials_provider = StaticProvider::new_minimal(access_key, secret_access_key);
        let request_dispatcher = HttpClient::new()
            .expect("Failed to create request dispatcher");

        // Return client
        Ok(S3Client::new_with(request_dispatcher, credentials_provider, region))
    }

    pub async fn test_image_upload(&self) -> Result<Asset> {
        // Set start
        use std::time::Instant;
        let start = Instant::now();

        // Set filename
        let filename = "nature-1-image.jpg";

        // Set path of sample upload
        let mut stream = match StdFile::open(format!("./assets/sample/{filename}")) {
            Ok(stream) => stream,
            Err(_) => return Err(Errors::internal_server_error(
                format!("Sample {filename} not found in path")
            ))
        };

        // Unwrap stream
        let mut contents: Vec<u8> = Vec::new();

        // Read file to end
        if stream.read_to_end(&mut contents).is_err() {
            return Err(Errors::internal_server_error("Unable to read file"));
        }

        // Retrieve result
        let result = self.upload_original(contents.clone(), filename).await?;

        // Print duration
        let duration = start.elapsed();
        println!("Time elapsed is: {duration:?}");

        // Return result
        Ok(result)
    }

    pub async fn upload_original<T>(&self, data: Vec<u8>, filename: T) -> Result<Asset>
        where T: ToString
    {
        // Bind filename
        let filename = filename.to_string();

        // Retrieve client
        let client = self.get_client()?;

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(&data.clone())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Set filename
        let legacy_filename = filename.clone();
        let extension = crate::parsers::ext_from_mime(mime.clone());

        // Create original path
        let bucket = self.bucket.clone();
        let key = format!("{}/original/{}", self.path.clone(),  filename.clone());

        // Upload original image to s3
        let request = PutObjectRequest {
            key,
            bucket,
            metadata: Some(metadata),
            body: Some(data.clone().into()),
            content_type: Some(mime.clone()),
            ..Default::default()
        };

        // Upload file
        if let Err(error) = client.put_object(request).await {
            return Err(Errors::internal_server_error(error));
        }

        // Check if data is image
        let mut w = None;
        let mut h = None;
        if Asset::is_image(&mime) {
            // Generate thumbnails concurrently
            if !self.get_thumbnail_sizes().is_empty() {
                let data_clone = data.clone();
                let s3 = self.clone();

                tokio::task::spawn(async move {
                    let mut img = data_clone;

                    for (width, height) in s3.get_thumbnail_sizes() {
                        if let Ok(result) = s3.generate_thumbnail(&img, &filename, width, height, false).await {
                            img = result;
                        }
                    }
                });

                // Note: Parallel but slower implementation because this uses the original image's resolution
                // for (width, height) in self.get_thumbnail_sizes() {
                //     let data_clone = Arc::new(data.clone());
                //     let s3_clone = Arc::new(self.clone());
                //     let filename_clone = Arc::new(filename.clone());
                //
                //     tokio::task::spawn(async move {
                //         let img = Arc::clone(&data_clone);
                //         let s3 = Arc::clone(&s3_clone);
                //         let fname = Arc::clone(&filename_clone);
                //
                //         _ = s3.generate_thumbnail(&img, &fname, width, height, false).await;
                //     });
                // }
            };

            // Load image from data
            let image = match image::load_from_memory(&data) {
                Ok(image) => image,
                Err(error) => return Err(Errors::internal_server_error(error))
            };

            // Calculate the size of the thumbnail
            let (orig_width, orig_height) = image.dimensions();
            w = Some(orig_width.to_string());
            h = Some(orig_height.to_string());
        }

        // Set file container
        let file = Asset{
            filename: Some(crate::parsers::change_ext(legacy_filename.clone(), "")),
            extension: Some(extension.to_string()),
            mime_type: Some(mime.clone()),
            width: w,
            height: h,
            file_size: Some(Asset::get_file_size(data.clone())),
            file_type: Some(Asset::get_file_type(&mime).to_string()),
            ..Default::default()
        };

        Ok(file)
    }

    pub async fn generate_thumbnail<T>(&self, data: &[u8], filename: T, width: u32, height: u32, retain_size: bool) -> Result<Vec<u8>>
        where T: ToString
    {
        // Create filename bindings
        let filename = crate::parsers::change_ext(filename, "webp");

        // Retrieve client
        let client = self.get_client()?;

        // Load image from data
        let mut image = match image::load_from_memory(data) {
            Ok(image) => image,
            Err(error) => return Err(Errors::internal_server_error(error))
        };

        // Convert image
        image = image::DynamicImage::from(image.to_rgba8());

        // Calculate the size of the thumbnail
        let (orig_width, orig_height) = image.dimensions();
        let ratio = f64::min( orig_width as f64 / width as f64, orig_height as f64 / height as f64);
        let new_width = (orig_width as f64 / ratio) as u32;
        let new_height = (orig_height as f64 / ratio) as u32;

        let mut thumbnail = match retain_size {
            true => image,
            false => image.resize(
                new_width,
                new_height,
                image::imageops::FilterType::Triangle
            )
        };

        // Crop the image to a square with the center as the gravity
        let (thumb_width, thumb_height) = thumbnail.dimensions();

        // Convert to f64
        let x:f64 = (thumb_width as f64 - width as f64) / 2.0;
        let y:f64 = (thumb_height as f64 - height as f64) / 2.0;

        // Round images to u32
        let x = x.round() as u32;
        let y = y.round() as u32;

        thumbnail = thumbnail.crop(x, y, width, height);

        // Add transparent padding if needed
        let mut padded_thumbnail = image::ImageBuffer::new(width, height);
        let transparent = image::Rgba([0, 0, 0, 0]);
        for (_, _, pixel) in padded_thumbnail.enumerate_pixels_mut() {
            *pixel = transparent;
        }

        // Set overlay
        image::imageops::overlay(&mut padded_thumbnail, &thumbnail, x as i64, y as i64);

        // Open the file and read its contents
        let mut cursor = std::io::Cursor::new(vec![]);
        if let Err(error) = thumbnail.write_to(&mut cursor, image::ImageFormat::WebP) {
            return Err(Errors::internal_server_error(error));
        }

        // Set buffer
        let buffer = cursor.get_ref();

        // Check out mime type
        let info = Infer::new();
        let mime = info
            .get(data.to_owned().as_slice())
            .map_or(String::default(), |t| String::from(t.mime_type()));

        // Set metadata
        let mut metadata = HashMap::new();
        metadata.insert(String::from("filename"), filename.clone());

        // Set path
        let path = self.path.clone();
        let path = format!("{path}/{width}x{height}/{filename}");

        // Upload original image to s3
        let request = PutObjectRequest {
            metadata: Some(metadata),
            bucket: self.bucket.clone(),
            key: path,
            body: Some(buffer.clone().into()),
            content_type: Some(mime),
            ..Default::default()
        };

        // Upload file
        if let Err(error) = client.put_object(request).await {
            return Err(Errors::internal_server_error(error));
        }

        Ok(buffer.clone())
    }
}