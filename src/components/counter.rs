use yew::prelude::*;
use stylist::{css, style};
use stylist::yew::{styled_component};

#[derive(Properties, PartialEq)]
pub struct CounterButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub label: String
}

#[styled_component(CounterButton)]
fn counter_button(props: &CounterButtonProps) -> Html {
    html! {
      <button
        class="counter-btn"
        onclick={props.onclick.clone()}
      >
        { props.label.clone() }
      </button>
    }
}

#[styled_component(Counter)]
pub fn counter() -> Html {
  let counter = use_state(|| 0);

  let increase = {
      let counter = counter.clone();
      Callback::from(move |_| counter.set(*counter + 1))
  };

  let decrease = {
    let counter = counter.clone();
    Callback::from(move |_| {
      let new_count = if *counter > 0 { *counter - 1 } else { 0 };
      counter.set(new_count)
    })
  };

  html! {
    <>
        <div class="counter">
          <div class="counter-actions">
            <CounterButton
              onclick={increase}
              label={"+1"}
            />
            <CounterButton
              onclick={decrease}
              label={"-1"}
            />
          </div>
          <p class="counter-num">{ *counter }</p>
        </div>
    </>
  }
}
