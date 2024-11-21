use leptos::*;

// https://book.leptos.dev/view/08_parent_child.html : 부모 컴포넌트, 자식 컴포넌트 연결
// https://book.leptos.dev/view/03_components.html?#components-and-props :  props 매크로 사용법
// callback 함수 prop
#[component]
pub fn TestComponent(
    #[prop(default = 100)] max: i32,
    calc_val: ReadSignal<i32>,
) -> impl IntoView {
    view! {
        <progress class="w-screen" max=max value=calc_val></progress>
        <p>{calc_val}</p>
    }
}
