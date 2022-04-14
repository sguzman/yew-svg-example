use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub enum Msg {
    AddOne,
    SubOne,
    NoOp,
    CheckBox
}

pub struct Model {
    value: i64,
    cond: bool
}

pub fn check(link: &yew::html::Scope<Model>, cond: bool) -> Html {
    html! {
        <input
            type="checkbox"
            onchange={link.callback(|_| Msg::CheckBox)}
            checked={cond} 
        />
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            cond: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
            Msg::SubOne => {
                self.value -= 1;
                true
            }
            Msg::NoOp => {
                false
            }
            Msg::CheckBox => {
                let msg = format!("Clicked box! Value currently is {}", !self.cond);
                log(msg.as_ref());
                self.cond = !self.cond;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                <button onclick={link.callback(|_| Msg::SubOne)}>{ "-1" }</button>
                if self.cond {
                    {check(link, true)}
                } else {
                    {check(link, false)}
                }
            </div>
        }
    }
}

