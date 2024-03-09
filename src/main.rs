mod components;

use leptos::{leptos_dom::logging::console_log, *};

use crate::components::{
    app_window::AppWindow, background::Background, flat_window::FlatWindow,
    progress_load::ProgressLoad, startup_load::StartupLoad,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    let (startup, set_startup) = create_signal(true);

    view! {
        <Background>

            <Show when=startup>
                <StartupLoad on_complete=move || { set_startup(false) }/>
            </Show>

            <Show when=move || !startup()>
                <AppWindow title=format!("Hello")>
                    <div>"hello"</div>
                </AppWindow>
            </Show>

        </Background>
    }
}
