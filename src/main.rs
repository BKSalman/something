use dioxus::{logger::tracing::Level, prelude::*};

use home::Home;
mod home;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home,
}

fn main() {
    dioxus::logger::init(Level::INFO).ok();

    #[cfg(feature = "web")]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                use dioxus_devtools::serve_subsecond;

                serve_subsecond(launch_router).await;
            });
    }
}

#[cfg(feature = "server")]
async fn launch_router() {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    let ip =
        dioxus::cli_config::server_ip().unwrap_or_else(|| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let port = dioxus::cli_config::server_port().unwrap_or(8080);
    let address = SocketAddr::new(ip, port);

    let router = axum::Router::new().serve_dioxus_application(ServeConfig::new().unwrap(), App);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    tracing::trace!("Listening on {address}");

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }

        Router::<Route> {}
    }
}
