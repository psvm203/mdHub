use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let markdown_icon: Html = html! {
        <div class={"flex justify-center"}>
            <img class={"w-64"} src="images/markdown.png" />
        </div>
    };

    let credit: Html = html! {
        <footer class={"fixed bottom-0 left-1/2 transform -translate-x-1/2"}
            role="contentinfo">
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
