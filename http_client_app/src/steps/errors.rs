extern crate async_step_machine;

use async_step_machine::{StepMachineError, StepError};

#[repr(i32)]
pub enum HttpClientError {
    FetchError = 0,
    EmptyResponse = 1,
    FailedFetchingFirstResponse = 2,
    FailedFetchingSecondResponse = 3
}

impl From<HttpClientError> for StepMachineError {
    fn from(error: HttpClientError) -> StepMachineError {
        StepMachineError::StepError(error as StepError)
    }
}