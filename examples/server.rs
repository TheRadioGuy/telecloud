use rusqlite::{NO_PARAMS, Connection};

pub fn main(){
    let connecton = Connection::open("./database/files.db").unwrap();
    let mut stmt = connecton.prepare("SELECT * FROM `main`").unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    while let Some(row) = rows.next().unwrap(){
        let s: i32 = row.get(5).unwrap();
        dbg!(s);
    }


}