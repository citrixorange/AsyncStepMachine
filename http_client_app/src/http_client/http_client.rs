use std::future::Future;
use std::pin::Pin;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::steps::errors::HttpClientError;

type ResponseFuture = Pin<Box<dyn Future<Output = Result<Option<String>,HttpClientError>>>>;

pub struct HttpResponse {
    response: ResponseFuture
}

impl HttpResponse {
    pub fn new(response: ResponseFuture) -> Self {
        Self {
            response: response
        }
    }

    pub async fn get_response(&mut self) -> Result<Option<String>,HttpClientError> {
        return self.response.as_mut().await;
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct HttpResponseData {
    body: String,
    id: u32,
    title: String,
    userId: u32,
}

pub struct FormattedResponse {
    formatted_response: Result<Option<String>,HttpClientError>
}

impl FormattedResponse {
    pub fn new() -> Self {
        Self {
            formatted_response: Ok(None)
        }
    }

    fn deserialized_format_response(&self) -> Vec<HttpResponseData> {
        match &self.formatted_response {
            Ok(response) => {
                if let Some(res) = response {
                    let deserialized: Vec<HttpResponseData> = serde_json::from_str(&res).unwrap();
                    return deserialized;
                } else {
                    return vec![];
                }
            }
            Err(_err) => {
                return vec![];
            }
        }
    }

    pub fn format_first_response(&mut self, response: Result<Option<String>,HttpClientError>) {
        match response {
            Ok(json_response) => {
                if let Some(json_res) = json_response {
                    let deserialized: HttpResponseData = serde_json::from_str(&json_res).unwrap();
                    self.formatted_response = Ok(Some(serde_json::to_string(&vec![deserialized]).unwrap()));
                } else {
                    self.formatted_response = Err(HttpClientError::EmptyResponse);
                }
            }
            Err(err) => {
                self.formatted_response = Err(err);
            }
        }
    } 

    pub fn format_second_response(&mut self, response: Result<Option<String>,HttpClientError>) {
        match response {
            Ok(json_response) => {
                if let Some(json_res) = json_response {
                    let mut deserialized_format_response: Vec<HttpResponseData> = self.deserialized_format_response();
                    let deserialized: HttpResponseData = serde_json::from_str(&json_res).unwrap();
                    deserialized_format_response.push(deserialized);
                    self.formatted_response = Ok(Some(serde_json::to_string(&deserialized_format_response).unwrap()));
                } else {
                    self.formatted_response = Err(HttpClientError::EmptyResponse);
                }
            }
            Err(err) => {
                self.formatted_response = Err(err);
            }
        }
    }

    pub fn print(&self) {
        match &self.formatted_response {
            Ok(response) => {
                if let Some(res) = response {
                    println!("{}", res);
                } else {
                    println!("None");
                }
            }
            Err(_err) => {
                println!("Error");
            }
        }
    }
}