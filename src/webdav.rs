use actix_web::{http, http::StatusCode, dev::HttpResponseBuilder, get, web, App, Error, HttpResponse, HttpServer, Responder, http::Method, HttpRequest, web::Bytes};

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
        _ => HttpResponse::Ok().status(StatusCode::BAD_REQUEST).body("Method not found").await.unwrap()
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
    HttpResponse::Ok().status(StatusCode::OK).header("DAV", "1,2").header("Allow", "OPTIONS,GET,HEAD,POST,DELETE,TRACE,PROPFIND,PROPPATCH,COPY,MOVE,LOCK,UNLOCK").finish().await.unwrap()
}

pub async fn handle_propfind(req: HttpRequest, body: String) -> HttpResponse {
    HttpResponse::Ok().status(StatusCode::MULTI_STATUS).header("DAV", "1,2").header("Allow", "OPTIONS,GET,HEAD,POST,DELETE,TRACE,PROPFIND,PROPPATCH,COPY,MOVE,LOCK,UNLOCK").body(r#"<?xml version="1.0" encoding="utf-8"?>
    <D:multistatus xmlns:D="DAV:" xmlns:ns0="DAV:">
    <D:response xmlns:lp1="DAV:" xmlns:lp2="http://apache.org/dav/props/" xmlns:g0="DAV:">
    <D:href>/webdav/</D:href>
    <D:propstat>
    <D:prop>
    <lp1:creationdate>2020-06-25T16:24:13Z</lp1:creationdate>
    <D:getcontenttype>httpd/unix-directory</D:getcontenttype>
    <lp1:getetag>"1000-5a8eb05e46786"</lp1:getetag>
    <lp1:getlastmodified>Thu, 25 Jun 2020 16:24:13 GMT</lp1:getlastmodified>
    <lp1:resourcetype><D:collection/></lp1:resourcetype>
    </D:prop>
    <D:status>HTTP/1.1 200 OK</D:status>
    </D:propstat>
    <D:propstat>
    <D:prop>
    <g0:displayname/>
    <g0:getcontentlength/>
    </D:prop>
    <D:status>HTTP/1.1 404 Not Found</D:status>
    </D:propstat>
    </D:response>
    </D:multistatus>"#).await.unwrap()
}
