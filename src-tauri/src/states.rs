use crate::core::Tasks;
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct TasksState(pub Arc<Mutex<Tasks>>);
