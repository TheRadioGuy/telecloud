
use std::path::PathBuf;

use crate::filestruct::{get_all_files, File};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use bytes::{Buf, BytesMut, Bytes};
use webdav_handler::actix::*;
use webdav_handler::fs::DavFileSystem;
use webdav_handler::{fakels::FakeLs, localfs::LocalFs, DavConfig, DavHandler};
use std::io::{Read, Seek};

pub async fn dav_handler(req: DavRequest, davhandler: web::Data<DavHandler>) -> DavResponse {
    let config = DavConfig::new().strip_prefix("/webdav");
    davhandler.handle_with(config, req.request).await.into()
}

#[derive(Clone)]
pub struct VirtualFs {}

use futures::future::FutureExt;
use std::time::SystemTime;
use webdav_handler::davpath::DavPath;
use webdav_handler::fs::DavFile;
use webdav_handler::fs::DavMetaData;
use webdav_handler::fs::FsFuture;
use webdav_handler::fs::FsResult;
use webdav_handler::fs::OpenOptions;
use std::io::{BufWriter, BufReader, Write};

// impl DavFileSystem for VirtualFs {
//     fn open(&self, path: &DavPath, options: OpenOptions) -> FsFuture<Box<VirtualFile>> {
//         async move {

//         }.boxed()
//     }
// }

#[derive(Debug)]
pub struct VirtualFile{
    path: PathBuf,
    buffer: Vec<u8>,
    metadata: Metadata,
    start_downloading: bool,
    filename: String,
    file: std::fs::File
}

impl VirtualFile {
    pub fn open(path: PathBuf) {

    }

    fn download_files(&mut self){
        info!("Starting to downloading");
    }
}
impl DavFile for VirtualFile {
    fn metadata(&mut self) -> FsFuture<Box<dyn DavMetaData>> {
        async move { Ok(Box::new(self.metadata.clone()) as Box<dyn DavMetaData>) }.boxed()
    }

    fn write_bytes(&mut self, buff: Bytes) -> FsFuture<()> {
        async move { self.file.write_all(&buff); Ok(()) }.boxed()
    }

    fn write_buf(&mut self, mut buf: Box<dyn bytes::buf::Buf + Send>) -> FsFuture<()> {
        async move {
                while buf.remaining() > 0 {
                    let n = self.file.write(buf.bytes()).unwrap();
                    buf.advance(n);
                }
                Ok(())
        }.boxed()
    }

    fn read_bytes(&mut self, count: usize) -> FsFuture<Bytes> {
        async move {

            if !self.start_downloading {
                self.start_downloading = true;
                self.download_files();
            }
            
            info!("Reading..");
            self.file = std::fs::File::create("stub.txt").unwrap(); // TODO: Downloading
            let mut buf = BytesMut::with_capacity(count);
            let res = unsafe {
                buf.set_len(count);
                self.file.read(&mut buf).map(|n| { buf.set_len(n); buf.freeze() })
            };
                
            res.map_err(|e| e.into())
        }.boxed()
    }

    fn seek(&mut self, pos: std::io::SeekFrom) -> FsFuture<u64> {
        async move {
            let mut file = &self.file;
            let (res, file) = { (file.seek(pos), file) };
            res.map_err(|e| e.into())
        }.boxed()
    }

    fn flush(&mut self) -> FsFuture<()>{
        async move {
            info!("FILE IS FLUSHED, WE CAN SAVE IT!");
            Ok(())
        }.boxed()
    }
}

#[derive(Clone, Debug)]
pub struct Metadata {
    pub length: u64,
    pub dir: bool,
}

impl DavMetaData for Metadata {
    fn len(&self) -> u64 {
        self.length
    }

    fn is_dir(&self) -> bool {
        self.dir
    }

    fn modified(&self) -> FsResult<SystemTime> {
        Ok(SystemTime::now()) // TODO: stub
    }
}
