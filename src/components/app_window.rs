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
pub fn AppWindow(children: ChildrenFn, title: String) -> impl IntoView {
    let window_state = create_rw_signal(WindowState {
        collapsed: false,
        minimized: false,
        width: 400.0,
        height: 200.0,
        x: 50.0,
        y: 50.0,
    });

    let (minimized, set_minimized) = create_slice(
        window_state,
        |state| state.minimized,
        |state, n| state.minimized = n,
    );

    let (collapsed, set_collapsed) = create_slice(
        window_state,
        |state| state.collapsed,
        |state, n| state.collapsed = n,
    );

    let (width, set_width) = create_slice(
        window_state,
        |state| state.width,
        |state, n| state.width = n,
    );

    let (height, set_height) = create_slice(
        window_state,
        |state| state.height,
        |state, n| state.height = n,
    );

    let (x, set_x) = create_slice(window_state, |state| state.x, |state, n| state.x = n);
    let (y, set_y) = create_slice(window_state, |state| state.y, |state, n| state.y = n);

    let children = store_value(children);

    view! {
        <div
            class="absolute bg-gray bevel-border-inner flex flex-col"
            style=move || {
                format!(
                    "top: {}px; left: {}px; height: {}px;",
                    y(),
                    x(),
                    if collapsed() { 40.0 } else { height() },
                )
            }
        >

            <div class=" h-[38px] px-2 flex gap-2 items-center" style=move || format!("width: {};", width())>
                <button class="close-button cursor-auto active:brightness-50"></button>
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

                <button class="close-button cursor-auto active:brightness-50">
                    <div class="mini-frame"></div>
                </button>
                <button class="close-button cursor-auto active:brightness-50" on:click=move |_| { set_collapsed(!collapsed()); }>
                    <div class="collapse-frame"></div>
                </button>
            </div>
            <Show when=move || !collapsed()>
                <div class="px-2 pb-2 flex-1 relative" >
                    <div class="bg-white bevel-border-outer h-full">{children.with_value(|children| children())}</div>
                </div>
            </Show>
        </div>
    }
}
