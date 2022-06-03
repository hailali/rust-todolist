use yew::{Children, Component, Context, Html, html};

use crate::components::counter::Counter;
use crate::components::task::Task;
use crate::components::task_manager::TaskManager;

pub struct Home {
    title: String,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            title: String::from("Welcome to my awesome application in RUST")
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <section class="section">
                <div class="container">
                    <h1 class="title">
                        { self.title.clone() }
                    </h1>
                    <p class="subtitle">
                        {"My first website with Bulma!"}
                    </p>
                    <TaskManager />
                </div>
            </section>
        }
    }
}
