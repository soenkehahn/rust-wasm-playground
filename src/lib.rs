use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

struct Model {
    link: ComponentLink<Self>,
    value: Vec<i32>,
}

enum Msg {
    Modify(fn(&mut Model)),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Modify(f) => f(self),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let list = self
            .value
            .iter()
            .map(|item: &i32| -> Html {
                html! {
                    <li> {item} </li>
                }
            })
            .collect::<Html>();

        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Modify(|model| model.value.push(model.value.len() as i32)))>{ "click me" }</button>
                <ul>
                    { list }
                </ul>
            </div>
        }
    }
}
