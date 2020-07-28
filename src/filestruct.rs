use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

pub static mut DATABASE: Option<Arc<Mutex<Connection>>> = None;

pub const CATALOG_STRUCTURE: &str = r#"CREATE TABLE IF NOT EXISTS `files` (
	`filename` VARCHAR(256),
	`mimetype` VARCHAR(16),
	`telegram_id` INT,
	`size` INT,
    `parts` VARCHAR,
    `vfs_path` VARCHAR
);"#;

pub fn upload_file(vfs_path: &str, file: PathBuf) {
    let vfs_path = vfs_path.to_owned();
    thread::spawn(move || {
        info!("Uploading file");
        let filename = file.file_name().unwrap().to_str().unwrap();

        let saved = File {
            filename: filename.to_string(),
            mimetype: "none".to_string(),
            telegram_id: 0,
            size: 0,
            parts: vec![],
            vfs_path
        };
        unsafe {
            info!("Send to channel..");
            crate::server::CHANNEL
                .as_mut()
                .unwrap()
                .lock()
                .unwrap()
                .send(SendedFile {
                    file: file,
                    info: saved,
                })
                .unwrap();
        }
    });
}

pub fn get_all_files(vfs_path: &str) -> Vec<File> {
    let mut files = Vec::new();
    let connecton = Connection::open("./database/files.db").unwrap();
    let mut stmt = connecton
        .prepare(&format!("SELECT * FROM `{}`", "files"))
        .unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let parts: String = row.get(4).unwrap();
        let vfs_path = row.get(5).unwrap();
        
        let parts = parts.split(",").map(|v| v.parse().unwrap_or(0)).collect();
        let file = File {
            filename: row.get(0).unwrap(),
            mimetype: row.get(1).unwrap(),
            telegram_id: row.get(2).unwrap(),
            size: row.get(3).unwrap(),
            parts,
            vfs_path
        };
        files.push(file);
    }

    files
}

pub fn get_file(vfs_path: &str) {}


pub fn database_init() {
    let db = Connection::open("./database/files.db").unwrap();

    db.execute(
        &CATALOG_STRUCTURE,
        NO_PARAMS,
    )
    .unwrap();

    let db = Arc::new(Mutex::new(db));
    unsafe {
        DATABASE = Some(db);
    }
}

#[derive(Debug)]
pub struct SendedFile {
    pub file: PathBuf,
    pub info: File,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub filename: String,
    pub mimetype: String,
    pub telegram_id: u32,
    pub size: u32,
    pub parts: Vec<i32>,
    pub vfs_path: String
}

impl File {
    pub fn insert(&self, conn: &mut Connection) {
        let mut parts = String::new();
        self.parts
            .iter()
            .for_each(|value| parts.push_str(&format!("{},", value)));
        info!("Parts: {}", parts);
        let req = &format!("INSERT INTO `files` (filename,mimetype,telegram_id,size,parts,vfs_path) values (?1, ?2, {}, {}, ?3, ?4)", self.telegram_id, self.size);
        info!("Request: {}", req);
        conn.execute(
            req,
            &[
                self.filename.clone(),
                self.mimetype.clone(),
                parts,
                self.vfs_path.clone()
            ],
        )
        .unwrap();
    }
}
