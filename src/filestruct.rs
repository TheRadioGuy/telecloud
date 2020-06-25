use ejdb::bson;
use ejdb::query::{Q, QH};
use ejdb::Database;
use image::imageops::FilterType;
use image::ImageFormat;
const IMAGES: &[&str] = &["png", "jpg", "jpeg"];
use actix_web::web;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::types::InputFile;
use tokio::runtime;
use uuid::Uuid;

pub static mut DATABASE: Option<Arc<RwLock<Database>>> = None;

pub fn upload_file(catalog: &str, file: PathBuf) {
    let catalog = catalog.to_owned();
    thread::spawn(move || {
        info!("Uploading file");
        let mut is_image = false;
        let extension = file.extension().unwrap().to_str().unwrap();
        IMAGES.iter().for_each(|filetype| {
            if &extension == filetype {
                is_image = true;
            }
        });

        let mut image_preview = None;

        if is_image {
            info!("It's an image. Resize it.");
            let img = image::open(&file).unwrap();
            let scaled = img.resize(50, 50, FilterType::Nearest); // make preview for imagecar
            let filename = file.file_name().unwrap().to_str().unwrap();
            let path = format!("./preview/preview_{}", filename);
            image_preview = Some(path.to_owned());
            info!("Path to preview: {}", path);
            let mut preview = std::fs::File::create(&path).unwrap();
            scaled.write_to(&mut preview, ImageFormat::Png);
        };

        let image_preview = image_preview.unwrap_or("none".to_owned());

        // telegram sending

        // let db = unsafe { &*DATABASE.as_mut().unwrap().write().unwrap() };
        // let coll = db.collection(catalog.clone()).unwrap();
        let file_cloned = file.clone();
        let filename = file.file_name().unwrap().to_str().unwrap();

        let file_save = bson! {
            "filename" => filename,
            "extension" => extension,
            "is_catalog" => "false",
            "telegram_id" => "100500",
            "size" => "0",
            "preview" => image_preview,
            "id" => "0",
            "parts" => "0"
        };

        // let id = coll.save(&file_save).unwrap().to_string();

        // info!("Saved to database");

        unsafe {
            info!("Send to channel..");
            crate::server::CHANNEL
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .send(SendedFile {
                    file: file_cloned,
                    info: file_save,
                    catalog: catalog.to_owned(),
                })
                .unwrap();
        }

        // let mut rt = runtime::Builder::new()
        //     .threaded_scheduler()
        //     .enable_all()
        //     .build()
        //     .unwrap();

        // rt.spawn(async move {
        //     let token = unsafe {
        //         crate::server::CONFIG
        //             .as_ref()
        //             .unwrap()
        //             .telegram_token
        //             .clone()
        //     };
        //     let chat_id = unsafe {
        //         crate::server::CONFIG
        //             .as_ref()
        //             .unwrap()
        //             .telegram_chatid
        //             .clone()
        //     };
        //     trace!("I'm spawned!");
        //     info!("Token: {}", token);
        //     let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050").unwrap();
        //     let bot = reqwest::Client::builder().proxy(proxy).build().unwrap();
        //     let bot = Bot::with_client(token, bot);
        //     info!("Sending document");
        //     let msg = bot.send_document(
        //         ChatId::Id(chat_id),
        //         InputFile::File(PathBuf::from(file_cloned)),
        //     );
        //     let msg = msg.send().await.unwrap();
        //     info!("Sended!");
        // });
    });
}

pub fn get_all_files(catalog: &str) -> Result<Vec<bson::Document>, Box<dyn std::error::Error>> {
    let db = unsafe { &*DATABASE.as_mut().unwrap().write().unwrap() };
    let coll = db.collection(catalog).unwrap();
    let q = coll
        .query(
            Q.field("filename").exists(true),
            QH.field("filename")
                .include()
                .field("extension")
                .include()
                .field("is_catalog")
                .include()
                .field("size")
                .include()
                .field("preview")
                .include()
                .field("_id")
                .include()
                .field("id")
                .include()
                .field("parts")
                .include(),
        )
        .find()?;

    let items: Result<Vec<bson::Document>, _> = q.collect();

    Ok(items?)
}

pub fn get_file(id: &str) {}

// TODO: Subcatalogs
pub fn make_catalog(current_catalog: String, catalog_name: String) {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());
    let db = unsafe { &*DATABASE.as_mut().unwrap().write().unwrap() };
    let coll = db.collection(current_catalog).unwrap();
    let cloned_name = catalog_name.clone();
    let cloned_id = id.clone();
    let file = bson! {
            "filename" => catalog_name,
            "extension" => "",
            "is_catalog" => "true",
            "telegram_id" => "0",
            "size" => "0",
            "preview" => "none",
            "id" => id,
            "parts" => "0"
    };

    let catalog_info = bson! {
        "uuid" => cloned_id,
        "name" => cloned_name
    };

    db.collection("_catalogs")
        .unwrap()
        .save(&catalog_info)
        .unwrap();

    let id = coll.save(&file).unwrap();
}

pub fn database_init() {
    let db = Arc::new(RwLock::new(
        Database::open("./database/files.json").unwrap(),
    ));
    unsafe {
        DATABASE = Some(db);
    }
}

#[derive(Debug)]
pub struct SendedFile {
    pub file: PathBuf,
    pub info: bson::Document,
    pub catalog: String,
}
