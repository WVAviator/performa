use leptos::*;

struct WindowState {
    collapsed: bool,
    minimized: bool,
    width: f32,
    height: f32,
    x: f32,
    y: f32,
}

#[component]
pub fn AppWindow(children: Children, title: String) -> impl IntoView {
    let (window_state, set_window_state) = create_signal(|| WindowState {
        collapsed: false,
        minimized: false,
        width: 200.0,
        height: 100.0,
        x: 50.0,
        y: 50.0,
    });

    view! {
        <div
            class="absolute bg-gray bevel-border-inner"
            style=move || {
                let state = window_state();
                format!(
                    "width: {}px; height: {}px; top: {}px; left: {}px;",
                    state().width,
                    state().height,
                    state().y,
                    state().x,
                )
            }
        >

            <div class="debug flex gap-1 p-1">
                <div class="bevel-concave w-[16px] h-[16px]"></div>
            </div>
            <div class="p-2">
                <div class="bg-white bevel-border-outer">{children()}</div>
            </div>
        </div>
    }
}
