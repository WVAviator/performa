use leptos::*;

#[component]
pub fn Background(children: Children) -> impl IntoView {
    view! {
        <div class="relative w-full h-full">
            <div
                class="w-full h-full"
                style="background-image: url(public/images/desktop_patterns/44.png); background-repeat: repeat;"
            >
                {children()}
            </div>
        </div>
    }
}
