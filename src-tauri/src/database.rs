use reqwest::Client;
use serde_json::{Value};
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct KeyWordMatch {
    pub key : String,
    pub match_ : String,
}
#[derive(Serialize,Deserialize)]
pub struct VectorMatch {
    pub vector: String,
    pub value: Vec<f32>,
    pub threshold: f32,
}

#[derive(Serialize,Deserialize)]
pub struct Filter {
    pub limit: u32,
    pub offset: Option<usize>,
    pub must : Option<Vec<KeyWordMatch>>,
    pub should : Option<Vec<VectorMatch>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub material: Option<String>,
    pub page: u32,
    pub pattern: Option<String>,
    pub rug_name: Option<String>,
    pub source: String,
    pub style: Option<String>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Payload {
    pub metadata: Option<Metadata>,
    pub page_content: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Point {
    pub id: String,
    pub payload: Payload
}


#[derive(Serialize,Deserialize,Debug)]
pub struct ApiResponse {
    pub next_page_offset: Option<String>,
    pub points: Vec<Point>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
    pub result: ApiResponse,
    pub status: String,
    pub time: f64,
}

pub async fn list_collections(client: &Client, base_url: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("{}", base_url);
    let res = client.get(&url).send().await?;

    let collections_list: Value = res.json().await?;
    println!("Collections: {:?}", collections_list);
    Ok(())
}

pub async fn get_collection_info(client: &Client, base_url: &str, collection_name: &str)
                                 -> Result<(), Box<dyn Error>> {
    let url = format!("{}/{}", base_url, collection_name);
    let res = client.get(&url).send().await?;

    let collection_info: Value = res.json().await?;
    println!("Collection Info: {:?}", collection_info);
    Ok(())
}


pub async fn list_points_with_filter(client: &Client, base_url: &str, collection_name: &str, filter: &Filter)
                                     -> Result<QueryResponse, Box<dyn Error>> {
    let url = format!("{}/{}/points/scroll", base_url, collection_name);

    let res = client.post(&url).json(filter).send().await?;

    let query_response: QueryResponse = res.json().await?;
    Ok(query_response)
}


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    // Ensure this is the correct URL for your running Qdrant instance
    const BASE_URL: &str = "http://localhost:6333/collections";

    #[tokio::test]
    async fn test_list_collections() -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let result = list_collections(&client, BASE_URL).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_collection_info() -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let collection_name = "Rug_sage"; // Ensure this collection exists or is set up before test
        let result = get_collection_info(&client, BASE_URL, collection_name).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_list_points_with_filter() -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let collection_name = "Rug_sage";
        let question = "What is the history of Persian rugs?";
        let embedding = crate::utils::generate_embedding_vector(question).await;
        // Ensure this collection exists or is set up before test
        let filter = Filter {
            limit: 2,
            offset: None,
            must:None,
            should:Some(vec![VectorMatch{
                vector:"text_embedding".to_string(),
                value: embedding,
                threshold:0.8
            }])
        };
        let result = list_points_with_filter(&client, BASE_URL, collection_name,&filter).await;
        assert!(result.is_ok());

        // Deserialize the result and inspect the points
        if let Ok(query_response) = result {
            println!("Test Points with Should Clause: {:?}", query_response);
            assert!(query_response.result.points.len() > 0, "No points returned");
            let content: Vec<_> = query_response
                .result
                .points.iter()
                .map(|page| page.payload.page_content.to_string())
                .collect();
            println!("page content: {:?}",content);
        }

        Ok(())
    }
}
