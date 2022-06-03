use yew::prelude::*;

use crate::components::home::Home;
mod components;

pub enum TaskStatusEnum {
    ACTIVE,
    DISABLE,
}

pub struct Task {
    pub id: String,
    pub name: String,
    pub status: TaskStatusEnum,
}

pub struct Task2 {
    pub id: String,
    pub name: String,
    pub status: TaskStatusEnum,
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<Home>();
}
