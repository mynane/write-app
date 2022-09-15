use crate::core::{Configs, Tasks};
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct TasksState(pub Arc<Mutex<Tasks>>);

#[derive(Default, Debug)]
pub struct ConfigsState(pub Arc<Mutex<Configs>>);
