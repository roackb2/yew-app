use super::counter::Counter;
use yew::prelude::*;
use stylist::yew::{styled_component};

#[derive(Properties, PartialEq)]
pub struct ContentProps {
    pub children: Children,
}

#[styled_component(Content)]
fn content(props: &ContentProps) -> Html {
    html! {
        <div id="content">
            { for props.children.iter() }
        </div>
    }
}

#[styled_component(App)]
pub fn app() -> Html {
  html! {
    <div id="app">
      <Content>
        <Counter />
      </Content>
    </div>
  }
}