use std::time::Duration;

use leptos::*;

#[component]
pub fn ProgressLoad<F>(text: &'static str, seconds: u64, on_complete: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    let (progress, set_progress) = create_signal(0);

    create_effect(move |_| {
        if progress() < 100 {
            set_timeout(
                move || {
                    set_progress(progress.get() + 10);
                },
                Duration::from_millis(seconds * 100),
            );
        } else {
            on_complete();
        }
    });

    view! {
        <div class="pt-4 flex flex-col items-center justify-center gap-2">
            <p class="text-sm font-thin font-chicago">{text}</p>
            <div class="bg-magenta border border-black border-solid w-[160px] h-[12px]">
                <div class="h-full bg-slate" style=move || format!("width: {}%", progress())></div>

            </div>
        </div>
    }
}
