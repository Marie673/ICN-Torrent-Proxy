use std::io;

pub async fn http_test() {
    let url = "http://www.net.info.hiroshima-cu.ac.jp";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .send()
        .await?;

    println!("{:#?}", response);
}