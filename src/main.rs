use yew::prelude::*;
use yew_router::prelude::*;

mod hello;
mod home;
mod rest;

use hello::Hello;
use home::Home;
use rest::Rest;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
    #[at("/404")]
    NotFound,
    #[at("/dbstuff")]
    RESTStuff,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Hello => html! { <Hello /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::RESTStuff => html! { <Rest /> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render = { switch } />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
