use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let markdown_icon: Html = html! {
        <img src="images/markdown.png" />
    };

    let credit: Html = html! {
        <footer role="contentinfo">
            <a href={"https://www.flaticon.com/free-icons/markdown"}
                title="markdown icon">
                {"Markdown icon created by Muhammad Andy - Flaticon"}
            </a>
        </footer>
    };

    html! {
        <>
            {markdown_icon}
            {credit}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
