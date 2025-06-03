use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let markdown_icon: Html = html! {
        <div class={"max-w-sm"}>
            <img class={"w-64"} src="images/markdown.png" />
        </div>
    };

    let introduction: Html = html! {
        <p class={"py-6"}>
            {"mdHub lets you open, edit, and commit Markdown files directly from your GitHub repositories.
                Works seamlessly in your browser with real-time editing and instant preview."}
        </p>
    };

    let login_button: Html = html! {
        <button class={"btn btn-primary"}>
            {"Continue with GitHub"}
        </button>
    };

    let hero: Html = html! {
        <div class={"hero bg-base-200 min-h-screen"}>
            <div class={"hero-content flex-col lg:flex-row"}>
                {markdown_icon}
                <div class={"max-w-md"}>
                    <h1 class={"text-5xl font-bold"}>{"mdHub"}</h1>
                    {introduction}
                    {login_button}
                </div>
            </div>
        </div>
    };

    let credit: Html = html! {
        <footer class={"footer sm:footer-horizontal footer-center bg-base-200 text-base-content p-4"}
            role="contentinfo">
            <a href={"https://www.flaticon.com/free-icons/markdown"}
                title="markdown icon">
                {"Markdown icon created by Muhammad Andy - Flaticon"}
            </a>
        </footer>
    };

    html! {
        <>
            {hero}
            {credit}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
