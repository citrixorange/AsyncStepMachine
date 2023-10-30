extern crate async_step_machine;

use std::sync::{Arc,RwLock};
use std::cell::RefCell;
use std::rc::Rc;

use async_step_machine::{StepMachineLabel, Step, StepMachine};
use crate::steps::labels::HttpClientSteps;
//use crate::steps::errors::{HttpClientError};
use crate::http_client::http_client::{HttpResponse, FormattedResponse};
use crate::steps::call::{
    fetch_first_response, 
    fetch_second_response, 
    format_first_response, 
    format_second_response
};

mod steps;
mod http_client;


#[tokio::main]
async fn main() {

    let steps:Vec<(StepMachineLabel,Box<Step<FormattedResponse,HttpResponse>>)>  = vec![
        (HttpClientSteps::FetchFirstResponse.into(), Box::new(|x,y| Box::pin(fetch_first_response(x,y)))),
        (HttpClientSteps::FetchSecondResponse.into(), Box::new(|x,y| Box::pin(fetch_second_response(x,y)))),
        (HttpClientSteps::FormatFirstResponse.into(), Box::new(|x,y| Box::pin(format_first_response(x,y)))),
        (HttpClientSteps::FormatSecondResponse.into(), Box::new(|x,y| Box::pin(format_second_response(x,y))))
    ];

    let vec_responses: Vec<Arc<RwLock<HttpResponse>>> = Vec::<Arc<RwLock<HttpResponse>>>::new();
    let formatted_response: FormattedResponse = FormattedResponse::new();
    let ptr_formatted_response = Rc::new(RefCell::new(formatted_response));
    let ptr_vec_response = Rc::new(RefCell::new(vec_responses));
    let mut step_machine = StepMachine::<FormattedResponse,HttpResponse>::new(Some(Rc::clone(&ptr_formatted_response)), Some(Rc::clone(&ptr_vec_response)), steps, None);
    let result = step_machine.run(HttpClientSteps::FetchFirstResponse.into()).await;
    assert!(result == Ok(()));
    let value = ptr_formatted_response.borrow();
    value.print();
    //assert!(value.result == Some(1));

}
