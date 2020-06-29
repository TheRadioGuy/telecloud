use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;
use rusqlite::{Connection};
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};


pub static mut DATABASE: Option<Arc<Mutex<Connection>>> = None;

pub const CATALOG_STRUCTURE: &str = r#"CREATE TABLE IF NOT EXISTS `{NAME}` (
	`id` VARCHAR(64) NOT NULL,
	`filename` VARCHAR(256),
	`mimetype` VARCHAR(16),
	`is_catalog` BOOLEAN,
	`telegram_id` INT,
	`size` INT,
	`parts` VARCHAR
);"#;

pub fn upload_file(catalog: &str, file: PathBuf) {
    let catalog = catalog.to_owned();
    thread::spawn(move || {
        info!("Uploading file");
        let filename = file.file_name().unwrap().to_str().unwrap();
        let id = format!("{}", Uuid::new_v4().to_hyphenated());

        let saved = File {id, filename: filename.to_string(), mimetype: "none".to_string(), is_catalog: false, telegram_id: 0, size: 0, parts: vec![]};
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
                    catalog: catalog.to_owned(),
                })
                .unwrap();
        }
    });
}

pub fn get_all_files(catalog: &str) -> Vec<File>{
    let mut files = Vec::new();
    let connecton = Connection::open("./database/files.db").unwrap();
    let mut stmt = connecton.prepare(&format!("SELECT * FROM `{}`", catalog)).unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    while let Some(row) = rows.next().unwrap(){
        let parts: String = row.get(6).unwrap();
        let parts = parts.split(",").map(|v| v.parse().unwrap_or(0)).collect();
        let file = File {
            id: row.get(0).unwrap(),
            filename: row.get(1).unwrap(),
            mimetype: row.get(2).unwrap(),
            is_catalog: row.get(3).unwrap(),
            telegram_id: row.get(4).unwrap(),
            size: row.get(5).unwrap(),
            parts,
        };
        files.push(file);
    }

    files
}

pub fn get_file(id: &str) {}

pub fn make_catalog(current_catalog: String, catalog_name: String) {
    let id = format!("{}", Uuid::new_v4().to_hyphenated());
    let mut connection = Connection::open("./database/files.db").unwrap();
    connection.execute( // make a new table
        &CATALOG_STRUCTURE.replace("{NAME}", &id),
        NO_PARAMS,
    ).unwrap();
    
    let file = File{id, filename: catalog_name, mimetype: "none".to_string(), is_catalog: true, telegram_id: 0, size: 0, parts: vec![]};
    file.insert(&current_catalog, &mut connection); // insert catalog into current one
}

pub fn database_init() {
    let db = Connection::open("./database/files.db").unwrap();

    db.execute( // make a MAIN catalog
        &CATALOG_STRUCTURE.replace("{NAME}", "main"),
        NO_PARAMS,
    ).unwrap();

    let db = Arc::new(Mutex::new(
        db
    ));
    unsafe {
        DATABASE = Some(db);
    }
}

#[derive(Debug)]
pub struct SendedFile {
    pub file: PathBuf,
    pub info: File,
    pub catalog: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub filename: String,
    pub mimetype: String,
    pub is_catalog: bool,
    pub telegram_id: u32,
    pub size: u32,
    pub parts: Vec<i32>
}

impl File {
    pub fn insert(&self, table: &str, conn: &mut Connection){
        let mut parts = String::new();
        self.parts.iter().for_each(|value| parts.push_str(&format!("{},", value)));
        info!("Parts: {}", parts);
        let req = &format!("INSERT INTO `{}` (id,filename,mimetype,is_catalog,telegram_id,size,parts) values (?1, ?2, ?3, {}, {}, {}, ?4)", table, self.is_catalog, self.telegram_id, self.size);
        info!("Request: {}", req);
        conn.execute(
            req,
            &[self.id.clone(), self.filename.clone(), self.mimetype.clone(), parts],
        ).unwrap();
        
    }
}
