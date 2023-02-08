use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::fetcher;
use crate::fetcher::FetchState;
use crate::Route;

async fn fetch_get_plain_text(url: &str) -> Result<String, gloo_net::Error> {
    fetcher::get(url).await?.text().await
}

pub enum Msg {
    SetFetchState(FetchState<String>),
    FetchHello,
    FetchAsyncDelay,
    FetchHelloName,
}
pub struct Hello {
    refs: Vec<NodeRef>,
    fetch_state: FetchState<String>,
}

impl Component for Hello {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            refs: vec![NodeRef::default(), NodeRef::default()],
            fetch_state: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.fetch_state = fetch_state;
                true
            }
            Msg::FetchHello => {
                ctx.link().send_future(async {
                    match fetch_get_plain_text("http://localhost:8000/hello").await {
                        Ok(hello) => Msg::SetFetchState(FetchState::Success(hello)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::FetchAsyncDelay => {
                let delay = self.refs[0].cast::<HtmlInputElement>().unwrap().value();
                ctx.link().send_future(async move {
                    match fetch_get_plain_text(
                        format!("http://localhost:8000/hello/delay/{}", delay).as_str(),
                    )
                    .await
                    {
                        Ok(hello) => Msg::SetFetchState(FetchState::Success(hello)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::FetchHelloName => {
                let input = self.refs[1].cast::<HtmlInputElement>().unwrap().value();
                let name = if input.is_empty() {
                    "Jimmy".to_owned()
                } else {
                    input
                };

                ctx.link().send_future(async move {
                    match fetch_get_plain_text(
                        format!("http://localhost:8000/hello/{}", name).as_str(),
                    )
                    .await
                    {
                        Ok(hello) => Msg::SetFetchState(FetchState::Success(hello)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hello_response = match &self.fetch_state {
            FetchState::NotFetching => "Not fetching",
            FetchState::Fetching => "Fetching",
            FetchState::Failed(_) => "Failed to fetch",
            FetchState::Success(data) => data.as_str(),
        };

        html! {
            <div>
                <h1 class="heart">{ "Hello" }</h1>
                <h3>{ hello_response }</h3>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::FetchHello)}>{ "Get hello world" }</button>
                </div>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::FetchAsyncDelay)}>{ "Get async delay" }</button>
                    <input
                        type="number"
                        ref={&self.refs[0]}
                        value={2}
                     />
                </div>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::FetchHelloName)}>{ "Get hello name" }</button>
                    <input
                        type="text"
                        ref={&self.refs[1]}
                        placeholder={"Jimmy"}
                    />
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
