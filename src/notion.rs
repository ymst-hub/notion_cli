use reqwest::Client;
use serde_json::json;

const NOTION_BASE_URL: &str = "https://api.notion.com/v1/";
const NOTION_VER: &str = "2022-06-28";

pub async fn auth_notion(secret_key: String, database_name: String){
    let client = Client::new();
    let url = format!(
        "{}{}{}",
        NOTION_BASE_URL, "databases/", database_name,
    );
    let response = client
        .get(url)
        //ヘッダー部
        .header(
            "Authorization",
            &format!("{} {}", "Bearer", &secret_key) as &str,
        )
        .header("Notion-Version", NOTION_VER)
        .send()
        .await
        .expect("api疎通エラー");
    let response_json = response.text().await.unwrap();
    println!("{}", response_json);
    
}

//全件取得
pub async fn fetch_notion_all_table(secret_key: String, database_name: String) {
    //検索内容
    let post_body = json! ({
            "filter":{
                "property": "tag",
            }
        }
    );
    
    let client = Client::new();
    let url = format!(
        "{}{}{}{}",
        NOTION_BASE_URL, "databases/", database_name, "/query"
    );
    let response = client
        .post(url)
        //ヘッダー部
        .header(
            "Authorization",
            &format!("{} {}", "Bearer", &secret_key) as &str,
        )
        .header("Notion-Version", NOTION_VER)
        .header("Content-Type", "application/json")

        .json(&post_body)
        .send()
        .await
        .expect("api疎通エラー");
    let response_json = response.text().await.unwrap();
    println!("{}", response_json);
}
