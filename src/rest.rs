use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

pub struct Rest;

pub enum Msg {

}

impl Component for Rest {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div>
                    <h1>{ "User stuff" }</h1>
                    <div>
                        <button>{ "Create user" }</button>
                    </div>
                </div>
                <div>
                    <Link<Route> to={Route::Home}>
                        <button>{ "Back to homepage" }</button>
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
