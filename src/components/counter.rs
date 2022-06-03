use yew::prelude::*;

pub enum Msg {
    AddOne,
    RemoveOne,
}

pub struct Counter {
    value: i64,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub initial_count: i64
}

impl Component for Counter {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: ctx.props().initial_count,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::RemoveOne => {
                self.value -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <button onclick={link.callback(|_| Msg::RemoveOne)}>{ "-1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
