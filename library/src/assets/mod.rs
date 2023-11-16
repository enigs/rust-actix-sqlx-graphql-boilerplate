use serde::{ Serialize, Deserialize };

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[derive(macros::SetIsEmpty)]
pub struct Asset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>
}

impl Asset {
    pub fn convert<T: serde::Serialize>(value: T) -> Self {
        serde_json::from_str(&serde_json::to_string(&value).unwrap_or_default())
            .unwrap_or(Asset::default())
    }

    #[allow(dead_code)]
    pub fn get_file_size(bytes: Vec<u8>) -> String {
        let mut size = bytes.len() as f64;
        let mut unit = "bytes";

        if size > 1024.0 {
            size /= 1024.0;
            unit = "KB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "MB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "GB";
        }

        if size > 1024.0 {
            size /= 1024.0;
            unit = "TB";
        }

        format!("{size:.2} {unit}")
    }

    /// Checks if mime type is image
    pub fn is_image(mime: &str) -> bool {
        let mimes = [
            "image/bmp",
            "image/gif",
            "image/jpg",
            "image/jpeg",
            "image/png",
            "image/webp",
            "image/x-icon",
            "image/x-tga",
            // "image/avif"
        ];

        mimes.contains(&mime.to_lowercase().as_str())
    }

    pub fn is_document(mime: &str) -> bool {
        let mimes = [
            "application/msword",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "application/pdf",
            "application/vnd.ms-excel",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "application/rtf",
            "application/vnd.oasis.opendocument.text",
            "application/vnd.oasis.opendocument.spreadsheet",
            "application/vnd.oasis.opendocument.presentation",
        ];

        mimes.contains(&mime.to_lowercase().as_str())
    }

    pub fn is_videos(mime: &str) -> bool {
        let mimes = [
            "video/mp4",
            "video/mpeg",
            "video/ogg",
            "video/webm",
            "video/x-flv",
            "video/x-ms-wmv",
            "video/quicktime",
            "video/3gpp",
            "video/3gpp2",
        ];

        mimes.contains(&mime.to_lowercase().as_str())
    }

    pub fn is_audio(mime: &str) -> bool {
        let mimes = [
            "audio/mpeg",
            "audio/ogg",
            "audio/webm",
            "audio/x-ms-wma",
            "audio/x-flac",
            "audio/aac",
        ];

        mimes.contains(&mime.to_lowercase().as_str())
    }

    pub fn is_xlsx(mime: &str) -> bool {
        let mimes = [
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "application/vnd.ms-excel.sheet.macroEnabled.12",
            "application/excel",
            "application/vnd.ms-excel",
            "application/x-excel",
            "application/x-msexcel",
            "application/vnd.ms-office",
            "application/vnd.ms-excel.sheet.macroEnabled.12",
            "application/vnd.ms-excel.sheet.binary.macroEnabled.12"
        ];

        mimes.contains(&mime.to_lowercase().as_str())
    }

    pub fn get_file_type(mime: &str) -> &str {
        match () {
            _ if Self::is_image(mime) => "image",
            _ if Self::is_xlsx(mime) => "spreadsheet",
            _ if Self::is_videos(mime) => "video",
            _ if Self::is_audio(mime) => "audio",
            _ if Self::is_document(mime) => "document",
            _ => "others"
        }
    }
}