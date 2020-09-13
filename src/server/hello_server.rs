use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::io;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

pub async fn run() -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(crate::SERVER_ADDRESS)?
        .run()
        .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test;
    use bytes::Bytes;

    #[actix_rt::test]
    async fn test_hello() {
        let mut app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello World"));
    }
}
