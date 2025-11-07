
use axum::body::Bytes;
use tokio::{fs::{self, File}, io::AsyncWriteExt};
use uuid::Uuid;

use crate::env::ENV;

fn replace_file_name_with_uuid(file_path: &str) -> String {
    // Get the file extension, if any
    let path = std::path::Path::new(file_path);
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    // Generate a new UUID
    let uuid = Uuid::new_v4();

    // Build new file name
    if extension.is_empty() {
        uuid.to_string()
    } else {
        format!("{}.{}", uuid, extension)
    }
}

pub async fn store_retreat_gallery(file_content: Bytes, file_name: String) -> String {
    let upload_dir = ENV.upload_dir.clone();
    let sub_dir = "retreat/gallery";
    let upload_full_path = upload_dir.join(sub_dir);
    fs::create_dir_all(&upload_full_path).await.ok();

    let updated_file_name = replace_file_name_with_uuid(&file_name);
    let relative_path = format!("{sub_dir}/{updated_file_name}");
    let gallery_path = upload_dir.join(&relative_path);
    println!("{gallery_path:?}");
    let mut file = File::create(gallery_path).await.unwrap();
    file.write_all(&file_content).await.unwrap();
    file.flush().await.unwrap();
    return relative_path;
}