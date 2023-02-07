use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div>
            <h1>{ "Homepage" }</h1>
            <div>
                <h3>{ "Basic stuff" }</h3>
                <div>
                    <Link<Route> to={Route::Hello}>
                        <button>{ "Greetings" }</button>
                    </Link<Route>>
                    <Link<Route> to={Route::NotFound}>
                        <button>{ "Nothing to see here" }</button>
                    </Link<Route>>
                </div>
            </div>
            <div>
                <h3>{ "Complicated stuff" }</h3>
                <div>
                    <Link<Route> to={Route::RESTStuff}>
                        <button>{ "REST API stuff" }</button>
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
