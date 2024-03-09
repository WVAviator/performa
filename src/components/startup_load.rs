use leptos::*;

use crate::components::{flat_window::FlatWindow, progress_load::ProgressLoad};

#[component]
pub fn StartupLoad<F>(on_complete: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {
        <FlatWindow class="flex flex-col justify-center items-center px-[36px] pt-[28px] pb-[20px] absolute top-[50%] left-[50%] -translate-x-[50%] -translate-y-[50%]">
            <div class="bg-white bevel-border-outer px-[70px] py-[18px]">
                <div class="w-[208px] h-[186px] flex flex-col items-center justify-center">
                    <img src="public/images/startup_logo.png" class="" alt="macos logo"/>
                </div>
            </div>
            <ProgressLoad text="Starting up..." seconds=1 on_complete=on_complete/>
        </FlatWindow>
    }
}
