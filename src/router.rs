use dioxus::prelude::*;
#[path = "./main.rs"]
mod main;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/mylist")]
    MyList {},
    #[route("/blog/")]
    Blog {},
}

#[component]
fn Blog() -> Element {
    rsx! {
        NavBar{}
        "Blog post"
    }
}

#[component]
fn MyList() -> Element {
    let mut text = "";
    rsx! {
        NavBar{}
        div {
            h1 {
                "MY LIST"
            }
            "imagine there's a fancy table view list here with options to sort by rating, site-wide rating, name, etc. it'll be really really cool promise :)"
            button { onclick: {move |_| text = " uhhh idk" }}
            "{text}"
        }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        div {
            Link { to: Route::Home {}, button { "Go Home"} }
            Link { to: Route::MyList {}, button { "Go to my list" } }
            Link { to: Route::Blog {}, button {"Go to blog"} }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        NavBar {}
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
