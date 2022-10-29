use reqwest::Client;

pub async fn fetch_notion_all_table(secret_key: String, database_name: String) {
    let client = Client::new();
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap())
}
