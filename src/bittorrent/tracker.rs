pub async fn http_test() -> Result<(), Box<dyn std::error::Error>>{
    let url = "http://www.net.info.hiroshima-cu.ac.jp";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .send()
        .await?;

    println!("{:#?}", response);
    Ok(())
}

