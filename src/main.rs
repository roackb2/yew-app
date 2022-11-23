use yew::{html, use_state, Callback};
use stylist::{css, style, StyleSource, YieldStyle};
use stylist::yew::{styled_component, Global};

#[styled_component(HelloWorld)]
fn hello_world() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let style = css!(r#"color: red;"#);

    html! {
        <div class={style}>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
     }
}

fn main() {
    yew::start_app::<HelloWorld>();
}