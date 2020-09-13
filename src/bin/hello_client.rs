use big_s::S;
use study_real_world_http::SERVER_URL;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = reqwest::get(SERVER_URL).await?;

    let body = res.text().await?;
    println!("{}", body);
    assert_eq!(body, S("Hello World"));
    Ok(())
}
