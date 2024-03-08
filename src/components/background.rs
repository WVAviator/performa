use leptos::*;

#[component]
pub fn Background(children: Children) -> impl IntoView {
    view! {
        <div class="relative w-full h-full">
            <div class="object-fill">
                <img src="%cr/static/desktop_patterns/44.png"/>
            </div>
            {children()}
        </div>
    }
}
