use super::counter::Counter;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub children: Children,
}

#[function_component(Content)]
fn content(props: &ContentProps) -> Html {
    html! {
        <div id="content">
            { for props.children.iter() }
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <div id="app">
      <Content>
        <Counter />
      </Content>
    </div>
  }
}