mod bittorrent;

use bittorrent::tracker;

fn main() {
    println!("ping");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn http_test() -> Result<(), Box<dyn std::error::Error>> {
        let url = "http://www.net.info.hiroshima-cu.ac.jp";
        let client = reqwest::Client::new();
        let response = client.get(url)
            .send().await?;

        let body = response.text().await?;

        println!("{}", body);

        Ok(())
    }
}


