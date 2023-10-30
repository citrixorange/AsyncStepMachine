extern crate async_step_machine;

use async_step_machine::{StepLabel, StepMachineLabel};

#[repr(i32)]
pub enum HttpClientSteps {
    FetchFirstResponse = 0,
    FetchSecondResponse = 1,
    FormatFirstResponse = 2,
    FormatSecondResponse = 3
}

impl From<HttpClientSteps> for StepMachineLabel {
    fn from(state: HttpClientSteps) -> StepMachineLabel {
        StepMachineLabel::StepLabel(state as StepLabel)
    }
}

impl From<HttpClientSteps> for StepLabel {
    fn from(state: HttpClientSteps) -> StepLabel {
        state as StepLabel
    }
}