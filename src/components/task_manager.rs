use yew::{Component, Context, events::Event, Html, html, TargetCast, MouseEvent, KeyboardEvent, InputEvent};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use crate::components::task::{Task, TaskStatusEnum};

pub enum Filter {
    All,
    Active,
    Completed,
}

pub enum TaskAction {
    Add,
    Nope,
    Update(String),
    Remove(usize),
    Toggle(usize),
    SetFilter(Filter),
}

pub struct TaskManager {
    tasks: Vec<Task>,
    filter: Filter,
    value: String
}

impl Component for TaskManager {
    type Message = TaskAction;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            tasks: vec![],
            filter: Filter::All,
            value: "".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let c = self.tasks.iter().count();

        match msg {
            TaskAction::Add => {
                log::info!("ADD: {:?}", self.value);
                self.tasks.push(
                    Task {
                        id: 2,
                        name: self.value.clone(),
                        status: TaskStatusEnum::ACTIVE,
                    }
                );

                self.value.clear();
                true
            }
            TaskAction::Update(taskName) => {
                log::info!("Update: {:?}", taskName);
                log::info!("Update self.value: {:?}", self.value);

                self.value = taskName;
                false
            }
            TaskAction::Nope => { false }
            TaskAction::Remove(taskIdx) => {
                self.tasks.remove(taskIdx);

                true
            }
            TaskAction::Toggle(taskIdx) => {
                let task = self.tasks.get_mut(taskIdx).unwrap();

                task.status = if task.status == TaskStatusEnum::COMPLETED { TaskStatusEnum::ACTIVE } else { TaskStatusEnum::COMPLETED };

                true
            }
            TaskAction::SetFilter(filter) => {
                self.filter = filter;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let total = self.tasks.len();
        let total_active = self.tasks.iter().filter(  |task| task.status == TaskStatusEnum::ACTIVE ).collect::<Vec<&Task>>().len();
        let total_completed = self.tasks.iter().filter(  |task| task.status == TaskStatusEnum::COMPLETED ).collect::<Vec<&Task>>().len();
        let html_tasks: Html = self.tasks.iter().filter(|task| {
            match self.filter {
                Filter::All => {
                    true
                }
                Filter::Active => {
                    task.status == TaskStatusEnum::ACTIVE
                }
                Filter::Completed => {
                    task.status == TaskStatusEnum::COMPLETED
                }
            }
        }).enumerate().map(|(idx, task)| {
            html! {
                <div class="block">
                    <div class="field is-grouped">
                        <div class="control is-expanded">
                            <Task id={String::from("12345")} name={task.name.clone()} status={task.status.clone()} />
                        </div>
                    <div>
                       
<a onclick={link.callback(move |_| TaskAction::Remove({idx}))}>
    <span class="icon">
    <i class="fa-solid fa-trash-can fa-lg"></i>
    </span>
    <span>{""}</span>
</a>

<a onclick={link.callback(move |_| TaskAction::Toggle({idx}))}>
<span class="icon">
<i class="fa-solid fa-circle-check fa-xl"></i>
</span>
<span>{""}</span>
</a> 
              
                        </div>
                    </div>
                </div>
            }
        }).collect();


            
        let on_press_enter_add_task = link.callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                TaskAction::Add
            } else {
                TaskAction::Nope   
            }
        });
        let on_dangerous_change = link.callback(|e: InputEvent| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            
            TaskAction::Update( target.unchecked_into::<HtmlInputElement>().value())
        });

        html! {
            <div>
                <input type="text" class="input" placeholder="What needs to be done?" value={self.value.clone()} name="task_name" oninput={on_dangerous_change} onkeypress={on_press_enter_add_task}  />
                <div class="box"> { html_tasks} </div>
                <div class="button_list">
                    <button class="button" onclick={link.callback(move |_| TaskAction::SetFilter(Filter::All))}> {"All"} </button>
                    <button class="button" onclick={link.callback(move |_| TaskAction::SetFilter(Filter::Active))}> {"All active"} </button>
                    <button class="button" onclick={link.callback(move |_| TaskAction::SetFilter(Filter::Completed))}> {"All completed"} </button>    
                </div>
                <footer class="footer">
                    <div class="content has-text-centered">
                        <span class="mgr-small tag is-success">{"Total :"} {total}</span>
                        <span class="mgr-small tag is-warning">{"Total active :"} {total_active}</span>
                        <span class="mgr-small tag is-info">{"Total completed :"} {total_completed}</span>
                    </div>
                </footer>
            </div>
        }
    }
}
