mod components;

use leptos::{leptos_dom::logging::console_log, *};

use crate::components::{
    background::Background, flat_window::FlatWindow, progress_load::ProgressLoad,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Background>
            <FlatWindow class="flex flex-col justify-center items-center px-[36px] pt-[28px] pb-[20px] absolute top-[50%] left-[50%] -translate-x-[50%] -translate-y-[50%]">
                <div class="bg-white bevel-border-outer px-[70px] py-[18px]">
                    <div class="w-[208px] h-[186px] flex flex-col items-center justify-center">

                        <img src="public/images/startup_logo.png" class="" alt="macos logo"/>
                    </div>
                </div>
                <ProgressLoad
                    text="Starting up..."
                    seconds=3
                    on_complete=move || { console_log("Loaded") }
                />
            </FlatWindow>
        </Background>
    }
}
