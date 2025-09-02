use dioxus::prelude::*;

#[server]
async fn button1(something: String) -> ServerFnResult<String> {
    Ok(something)
}

#[server]
async fn button2(something: String) -> ServerFnResult<String> {
    Ok(something)
}

#[component]
pub fn Home() -> Element {
    let x = 21;

    rsx! {
        h1 {"{x}"}

        button {
            onclick: move |_| async move{
                if let Err(e) = button1(String::from("button1")).await {
                    tracing::error!("{e}");
                }
            },
            "button1"
        }
        button {
            onclick: move |_| async move{
                if let Err(e) = button2(String::from("button2")).await {
                    tracing::error!("{e}");
                }
            },
            "button2"
        }
    }
}
