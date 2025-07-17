use reqwest::Client;

pub async fn execute_get_request(target_url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    //SINK
    let response = client.get(&target_url).send().await?;
    let body = response.text().await?;
    
    Ok(body)
}

pub async fn execute_post_request(target_url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let payload = serde_json::json!({"data": "test"});
    
    //SINK
    let response = client.post(&target_url).json(&payload).send().await?;
    let body = response.text().await?;
    
    Ok(body)
}

pub async fn process_http_requests(get_url: String, post_url: String) -> Result<(), Box<dyn std::error::Error>> {
    execute_get_request(get_url).await?;
    execute_post_request(post_url).await?;
    Ok(())
} 