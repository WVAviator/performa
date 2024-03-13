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
        width: 400.0,
        height: 200.0,
        x: 50.0,
        y: 50.0,
    });

    view! {
        <div
            class="absolute scale-[0.5] bg-gray bevel-border-inner"
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

            <div class=" h-[38px] px-2 flex gap-2 items-center">
                <button class="close-button"></button>
                <div class="flex-grow flex flex-col">
                    {(0..6)
                        .map(|_| {
                            view! {
                                <div class="">
                                    <div class="pr-[2px]">
                                        <div class="h-[2px] bg-[#DDDDDD]"></div>
                                    </div>
                                    <div class="pl-[2px]">
                                        <div class="h-[2px] bg-[#999999]"></div>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}

                </div>
                <h2 class="m-0 font-chicago text-slate text-[20px]">{title}</h2>
                <div class="flex-grow flex flex-col">
                    {(0..6)
                        .map(|_| {
                            view! {
                                <div class="">
                                    <div class="pr-[2px]">
                                        <div class="h-[2px] bg-[#DDDDDD]"></div>
                                    </div>
                                    <div class="pl-[2px]">
                                        <div class="h-[2px] bg-[#999999]"></div>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}

                </div>

                <button class="close-button">
                    <div class="mini-frame"></div>
                </button>
                <button class="close-button">
                    <div class="collapse-frame"></div>
                </button>
            </div>
            <div class="px-2">
                <div class="bg-white min-h-[100px] bevel-border-outer">{children()}</div>
            </div>
        </div>
    }
}
