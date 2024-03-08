mod components;

use leptos::*;

use crate::components::background::Background;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <Background>
            <button
                class="py-4 px-6 bg-red-200 text-lg"
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
            >

                "Click me: "
                {move || count()}
            </button>
        </Background>
    }
}
