use yew::{Component, Context, Html, html, Properties};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatusEnum {
    ACTIVE,
    COMPLETED
}

pub enum TaskSAction {
    ACTIVE,
    COMPLETED
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: String,
    pub name: String ,
    pub status: TaskStatusEnum
}

pub struct Task {
    pub id: usize,
    pub name: String,
    pub status: TaskStatusEnum
}

impl Component for Task {
    type Message = TaskStatusEnum;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
       Self {
           id: 123,
           name: ctx.props().name.clone(),
           status: TaskStatusEnum::ACTIVE,
       }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        let class =  if props.status == TaskStatusEnum::COMPLETED {{"text-decoration: line-through;"}} else {{""}};
        html! {
            <input type="text" class="input" value={props.name.clone()} style={{class}} />
        }
    }
}


