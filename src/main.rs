use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter: UseStateHandle<i32> = use_state(|| 0);
    let add = {
        let counter:UseStateHandle<i32> = counter.clone();
        move |_| {
            let value:i32 = *counter + 1;
            counter.set(value);
        }
    };
    let sub = {
        let counter:UseStateHandle<i32> = counter.clone();
        move |_| {
            let value:i32 = *counter - 1;
            counter.set(value);
        }
    };
    html! {
        <div>
            <button onclick= {add}>{ "+" }</button>
            <p>{ *counter }</p>
            <button onclick= {sub}>{ "-" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}