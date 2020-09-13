#[actix_web::main]
async fn main() -> std::io::Result<()> {
    study_real_world_http::server::hello_server::run().await?;
    Ok(())
}
