use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: String,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value = format!("{}a", self.value),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ &self.value }</p>
                <Foo count={self.value.len()} />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}

#[derive(Clone, Properties)]
struct FooProps {
    count: usize,
}

struct Foo {
    len: usize,
}

struct Never {
    _never: &'static Never,
}

impl Component for Foo {
    type Message = Never;
    type Properties = FooProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Foo { len: props.count }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.len = props.count;
        true
    }

    fn view(&self) -> Html {
        html! {
            {format!("this is Foo: {}", self.len)}
        }
    }
}
