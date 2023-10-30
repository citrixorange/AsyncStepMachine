use std::cmp::PartialEq;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::cell::RefCell;
use std::rc::Rc;
use std::future::Future;
use std::pin::Pin;

pub type StepLabel = i32;
pub type StepError = i32;

#[derive(Eq, PartialEq, Hash)]
pub enum StepMachineLabel {
    Done,
    StepLabel(StepLabel)
}

#[derive(Eq, PartialEq)]
pub enum StepMachineError {
    InternalError,
    InexistentStep,
    StepError(StepError)
}

pub type Step<T,V> = dyn Fn(Option<Rc<RefCell<T>>>, Option<Rc<RefCell<Vec<Arc<RwLock<V>>>>>>) -> Pin<Box<dyn Future<Output = Result<Option<StepMachineLabel>,StepMachineError>>>>;

pub type ErrorHandler<T,V> = dyn Fn(StepLabel, StepMachineError, Option<Rc<RefCell<T>>>, Option<Rc<RefCell<Vec<Arc<RwLock<V>>>>>>) -> Pin<Box<dyn Future<Output = StepMachineError>>>;

pub struct StepMachine<T,V>
{
    sync_handler: Option<Rc<RefCell<T>>>,
    async_handlers: Option<Rc<RefCell<Vec<Arc<RwLock<V>>>>>>,
    steps: HashMap<StepMachineLabel,Box<Step<T,V>>>,
    error_handler: Option<Box<ErrorHandler<T,V>>>
}

impl<T,V> StepMachine<T,V>
{
    pub fn new(sync_handler:Option<Rc<RefCell<T>>>, async_handlers:Option<Rc<RefCell<Vec<Arc<RwLock<V>>>>>>, steps: Vec<(StepMachineLabel,Box<Step<T,V>>)>, error_handler: Option<Box<ErrorHandler<T,V>>>) -> Self {
        Self {
            sync_handler: sync_handler,
            async_handlers: async_handlers,
            steps: steps.into_iter().collect(),
            error_handler: error_handler
        }
    }

    fn clone_handlers(&self) -> (Option<Rc<RefCell<T>>>,Option<Rc<RefCell<Vec<Arc<RwLock<V>>>>>>) {
        (
            self.sync_handler.as_ref().map(|h| h.clone()),
            self.async_handlers.as_ref().map(|h| h.clone()),
        )
    }

    pub async fn run(&mut self, beginning:StepMachineLabel) -> Result<(),StepMachineError> {

        let mut last_step = beginning;

        while let Some(step) = self.steps.get(&last_step) {

            let (sync_handler,async_handlers) = self.clone_handlers();

            match step(sync_handler, async_handlers).await {

                Ok(Some(StepMachineLabel::Done)) => return Ok(()),

                Ok(Some(next_step)) => {
                    last_step = next_step;
                }

                Ok(None) => return Ok(()),
                
                Err(error_code) => {
                    if let Some(err_handler) = &self.error_handler {
                        if let StepMachineLabel::StepLabel(last_step_label) = last_step {
                            let (sync_handler,async_handlers) = self.clone_handlers();
                            return Err(err_handler(last_step_label,error_code,sync_handler,async_handlers).await);
                        } else {
                            return Err(StepMachineError::InternalError);
                        }
                    }
                    return Err(error_code);
                }
            }
        }
    
        return Err(StepMachineError::InexistentStep);

    }
}