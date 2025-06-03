use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let markdown_icon: Html = html! {
        <div class={"max-w-sm"}>
            <img class={"w-64"} src={"images/markdown.png"} />
        </div>
    };

    let title: Html = html! {
        <h1 class={"text-5xl font-bold"}>{"Edit your Markdown files."}</h1>
    };

    let introduction: Html = html! {
        <p class={"py-6"}>
            <a class={"link"} href={"https://github.com/psvm203/mdHub"}>
                {"mdHub"}
            </a>
            {" is a web-based tool designed to simplify working with Markdown files in your GitHub repositories.
                You can browse, edit, preview, and commit Markdown files with ease."}
        </p>
    };

    let login_button: Html = html! {
        <button class={"btn bg-black text-white border-black"}>
            <img class={"w-6"} src={"images/github-mark-white.svg"} />
            {"Continue with GitHub"}
        </button>
    };

    let hero: Html = html! {
        <div class={"hero bg-base-200 min-h-[95vh]"}>
            <div class={"hero-content flex-col lg:flex-row"}>
                {markdown_icon}
                <div class={"max-w-md"}>
                    {title}
                    {introduction}
                    {login_button}
                </div>
            </div>
        </div>
    };

    let credit: Html = html! {
        <footer class={"footer sm:footer-horizontal footer-center bg-base-300 text-base-content min-h-[5vh]"}
            role={"contentinfo"}>
            <a href={"https://www.flaticon.com/free-icons/markdown"}
                title={"markdown icon"}>
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
