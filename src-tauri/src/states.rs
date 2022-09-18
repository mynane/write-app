use crate::core::{Configs, Repositories, Tasks};
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct TasksState(pub Arc<Mutex<Tasks>>);

#[derive(Default, Debug)]
pub struct ConfigsState(pub Arc<Mutex<Configs>>);

#[derive(Default, Debug)]
pub struct RepositoriesState(pub Arc<Mutex<Repositories>>);
