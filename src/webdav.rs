const XML_TEMPLATE: &str = include_str!("../compile/response_template.xml");
const XML_ENTRY: &str = include_str!("../compile/response_entry.xml");
const XML_ENTRY_DIR: &str = include_str!("../compile/response_entry_dir.xml");


use actix_web::{http::StatusCode, web::Bytes, HttpRequest, HttpResponse};
use crate::filestruct::{File, get_all_files};

use std::path::PathBuf;

pub async fn webdav_handle(req: HttpRequest, body: Bytes) -> HttpResponse {
    println!("----------------------------");
    println!("----------------------------");
    println!("----------------------------");

    dbg!(&req);
    let method = req.method().as_str();
    let text = String::from_utf8(body.to_vec()).unwrap();
    println!("{}", text);

    let response = match method {
        "OPTIONS" => handle_option(req, text).await,
        "PROPFIND" => handle_propfind(req, text).await,
        _ => {error!("Method not found : {}", method); HttpResponse::Ok()
            .status(StatusCode::BAD_REQUEST)
            .body("Method not found")
            .await
            .unwrap()},
    };

    response

    //     HttpResponse::Ok().status(StatusCode::MULTI_STATUS).set_header(http::header::CONTENT_TYPE, "text/xml").body(r#"<D:multistatus
    //         xmlns:D="DAV:"
    //         xmlns:ns1="http://apache.org/dav/props/"
    //         xmlns:ns0="DAV:">
    //         <D:response
    //             xmlns:lp1="DAV:"
    //             xmlns:lp2="http://apache.org/dav/props/"
    //             xmlns:g0="DAV:"
    //             xmlns:g1="http://apache.org/dav/props/">
    //             <D:href>
    //                 /webdav/
    //                 </D:href>
    //             <D:propstat>
    //                 <D:prop>
    //                     <lp1:getlastmodified>
    //                         Thu, 25 Jun 2020 15:09:37 GMT
    //                         </lp1:getlastmodified>
    //                     <lp1:resourcetype>
    //                         <D:collection/>
    //                         </lp1:resourcetype>
    //                     </D:prop>
    //                 <D:status>
    //                     HTTP/1.1 200 OK
    //                     </D:status>
    //                 </D:propstat>
    //             <D:propstat>
    //                 <D:prop>
    //                     <g0:getcontentlength/>
    //                     <g1:executable/>
    //                     <g0:checked-in/>
    //                     <g0:checked-out/>
    //                     </D:prop>
    //                 <D:status>
    //                     HTTP/1.1 404 Not Found
    //                     </D:status>
    //                 </D:propstat>
    //             </D:response>
    //         </D:multistatus>
    // "#)
}

pub async fn handle_option(req: HttpRequest, body: String) -> HttpResponse {
    info!("Option, handle it!");
    HttpResponse::Ok()
        .status(StatusCode::OK)
        .header("DAV", "1,2")
        .header(
            "Allow",
            "OPTIONS,GET,HEAD,POST,DELETE,TRACE,PROPFIND,PROPPATCH,COPY,MOVE,LOCK,UNLOCK",
        )
        .finish()
        .await
        .unwrap()
}

pub async fn handle_propfind(req: HttpRequest, body: String) -> HttpResponse {
    let template = files_to_xml(&get_all_files("main"), "/");
    HttpResponse::Ok()
        .status(StatusCode::MULTI_STATUS)
        .header("DAV", "1,2")
        .header(
            "Allow",
            "OPTIONS,GET,HEAD,POST,DELETE,TRACE,PROPFIND,PROPPATCH,COPY,MOVE,LOCK,UNLOCK",
        )
        .body(template)
        .await
        .unwrap()
}

// Current catalog is NAME, not a UUID!
fn files_to_xml(files: &Vec<File>, current_catalog: &str) -> String {
    let mut response = XML_TEMPLATE.replace("{CURRENT_CATALOG}", &format!("/webdav{}", current_catalog));
    let mut other_files = String::new();

    files.iter().for_each(|file| {
        let path_to_file = format!("/webdav{}{}", current_catalog, file.filename);
        let template = match file.is_catalog {
            true => XML_ENTRY_DIR.replace("{PATH_TO_FILE}", &path_to_file).replace("ETAG", "change me dude"),
            false => XML_ENTRY.replace("{PATH_TO_FILE}", &path_to_file).replace("{SIZE}", &file.size.to_string()).replace("{TYPE}", "text/x-dsrc").replace("ETAG", "change me dude")

        };
        other_files.push_str(&template);
    });

    let response = response.replace("{OTHER_FILES}", &other_files);

    info!("GENERATED XML:");
    info!("{}", response);

    response
}