use leptos::*;

#[component]
pub fn FlatWindow(children: Children, class: &'static str) -> impl IntoView {
    let class = format!("bg-gray bevel-border-inner {}", class);
    view! { <div class=class>{children()}</div> }
}
