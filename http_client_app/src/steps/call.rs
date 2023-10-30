extern crate async_step_machine;

use std::sync::{Arc, RwLock};
use std::cell::RefCell;
use std::rc::Rc;

use serde_json::Value;
use reqwest;

use async_step_machine::{StepMachineLabel, StepMachineError};
use crate::steps::labels::HttpClientSteps;
use crate::steps::errors::HttpClientError;
use crate::http_client::http_client::{HttpResponse, FormattedResponse};

pub async fn fetch_first_response(_sync_handler: Option<Rc<RefCell<FormattedResponse>>>, async_handlers: Option<Rc<RefCell<Vec<Arc<RwLock<HttpResponse>>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {

    let future_response = async {

        let url = "https://jsonplaceholder.typicode.com/posts/1";
        let response = reqwest::get(url).await.map_err(|_| HttpClientError::FailedFetchingFirstResponse)?;
    
        if response.status().is_success() {
            let json_value: Value = response.json().await.map_err(|_| HttpClientError::FailedFetchingFirstResponse)?;
            let serialized_json = serde_json::to_string(&json_value).map_err(|_| HttpClientError::FailedFetchingFirstResponse)?;
            return Ok(Some(serialized_json));
        } else {
            return Err(HttpClientError::FailedFetchingFirstResponse);
        }
    };

    if let Some(handlers) = async_handlers {
        let mut async_handler_ref = handlers.borrow_mut();
        async_handler_ref.push(Arc::new(RwLock::new(HttpResponse::new(Box::pin(future_response)))));
    } else {
        return Err(HttpClientError::FailedFetchingFirstResponse.into());
    }

    return Ok(Some(HttpClientSteps::FetchSecondResponse.into()));
}

pub async fn fetch_second_response(_sync_handler: Option<Rc<RefCell<FormattedResponse>>>, async_handlers: Option<Rc<RefCell<Vec<Arc<RwLock<HttpResponse>>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    
    let future_response = async {

        let url = "https://jsonplaceholder.typicode.com/posts/2";
        let response = reqwest::get(url).await.map_err(|_| HttpClientError::FailedFetchingFirstResponse)?;
    
        if response.status().is_success() {
            let json_value: Value = response.json().await.map_err(|_| HttpClientError::FailedFetchingFirstResponse)?;
            let serialized_json = serde_json::to_string(&json_value).map_err(|_| HttpClientError::FailedFetchingFirstResponse)?;
            return Ok(Some(serialized_json));
        } else {
            return Err(HttpClientError::FailedFetchingFirstResponse);
        }
    };

    if let Some(handlers) = async_handlers {
        let mut async_handler_ref = handlers.borrow_mut();
        async_handler_ref.push(Arc::new(RwLock::new(HttpResponse::new(Box::pin(future_response)))));
    } else {
        return Err(HttpClientError::FailedFetchingFirstResponse.into());
    }
    
    return Ok(Some(HttpClientSteps::FormatFirstResponse.into()));
}

pub async fn format_first_response(sync_handler: Option<Rc<RefCell<FormattedResponse>>>, async_handlers: Option<Rc<RefCell<Vec<Arc<RwLock<HttpResponse>>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {

    if let Some(handlers) = async_handlers {
        let async_handler_ref = handlers.borrow();
        let handler = Arc::clone(&async_handler_ref[0]);
        let mut http_response = handler.write().unwrap();
        if let Some(handler) = &sync_handler {
            let mut format_response = handler.borrow_mut();
            let response = http_response.get_response().await;
            format_response.format_first_response(response);
        }
    } 

    return Ok(Some(HttpClientSteps::FormatSecondResponse.into()));
}

pub async fn format_second_response(sync_handler: Option<Rc<RefCell<FormattedResponse>>>, async_handlers: Option<Rc<RefCell<Vec<Arc<RwLock<HttpResponse>>>>>>) -> Result<Option<StepMachineLabel>,StepMachineError> {
    if let Some(handlers) = async_handlers {
        let async_handler_ref = handlers.borrow();
        let handler = Arc::clone(&async_handler_ref[1]);
        let mut http_response = handler.write().unwrap();
        if let Some(handler) = &sync_handler {
            let mut format_response = handler.borrow_mut();
            let response = http_response.get_response().await;
            format_response.format_second_response(response);
        }
    } 
    return Ok(Some(StepMachineLabel::Done));
}