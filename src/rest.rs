use std::collections::HashMap;
use dotenv_codegen::dotenv;
use gloo_net::http::Response;
use maplit::hashmap;
use rgb::RGB8;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::fetcher;
use crate::{fetcher::FetchState, Route};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    name: String,
    color: RGB8,
}

struct Color(RGB8);

impl From<String> for Color {
    fn from(value: String) -> Self {
        let bytes = hex::decode(value.as_str()[1..].to_string()).unwrap();
        Self(RGB8 {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
        })
    }
}

pub struct Rest {
    refs: HashMap<&'static str, NodeRef>,
    get_state: FetchState<User>,
    post_state: FetchState<Response>,
    delete_state: FetchState<Response>,
}

pub enum Msg {
    PostUser,
    DeleteUser,
    GetUser,
    SetGetState(FetchState<User>),
    SetPostState(FetchState<Response>),
    SetDeleteState(FetchState<Response>),
}

impl Component for Rest {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            refs: hashmap! {
                "username" => NodeRef::default(),
                "color" => NodeRef::default(),
                "delete-id" => NodeRef::default(),
                "get-id" => NodeRef::default(),
            },
            post_state: FetchState::NotFetching,
            delete_state: FetchState::NotFetching,
            get_state: FetchState::NotFetching,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPostState(post_state) => {
                self.post_state = post_state;
                true
            }
            Msg::SetDeleteState(delete_state) => {
                self.delete_state = delete_state;
                true
            }
            Msg::SetGetState(get_state) => {
                self.get_state = get_state;
                true
            }
            Msg::GetUser => {
                let id = self
                    .refs
                    .get("get-id")
                    .unwrap()
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                ctx.link().send_future(async move {
                    match fetcher::get_object::<User>(
                        format!(
                            "{}/{}/user/{}",
                            dotenv!("BASE_ADDRESS"),
                            dotenv!("API_PATH"),
                            id
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(res) => Msg::SetGetState(FetchState::Success(res)),
                        Err(err) => Msg::SetGetState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetGetState(FetchState::Fetching));
                false
            }
            Msg::PostUser => {
                let color: Color = self
                    .refs
                    .get("color")
                    .unwrap()
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .into();
                let user = User {
                    name: self
                        .refs
                        .get("username")
                        .unwrap()
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value(),
                    color: color.0,
                };
                // web_sys::console::log_1(&serde_wasm_bindgen::to_value(&user).unwrap());
                ctx.link().send_future(async move {
                    match fetcher::post(
                        format!("{}/{}/user", dotenv!("BASE_ADDRESS"), dotenv!("API_PATH"))
                            .as_str(),
                        user,
                    )
                    .await
                    {
                        Ok(res) => Msg::SetPostState(FetchState::Success(res)),
                        Err(err) => Msg::SetPostState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetPostState(FetchState::Fetching));
                false
            }
            Msg::DeleteUser => {
                let id = match self
                    .refs
                    .get("delete-id")
                    .unwrap()
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .parse::<u32>()
                {
                    Ok(id) => id,
                    Err(err) => {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message(&err.to_string())
                            .unwrap();
                        return false;
                    }
                };
                ctx.link().send_future(async move {
                    match fetcher::delete(
                        format!(
                            "{}/{}/user/{}",
                            dotenv!("BASE_ADDRESS"),
                            dotenv!("API_PATH"),
                            id
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(res) => Msg::SetDeleteState(FetchState::Success(res)),
                        Err(err) => Msg::SetDeleteState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetDeleteState(FetchState::Fetching));
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.delete_state {
            FetchState::NotFetching => (),
            FetchState::Fetching => (),
            FetchState::Failed(err) => web_sys::window()
                .unwrap()
                .alert_with_message(
                    format!("Delete request failed; Reason: {}", err.to_string()).as_str(),
                )
                .unwrap(),
            FetchState::Success(res) => {
                if res.ok() {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("Deleted")
                        .unwrap()
                } else {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("No such user id")
                        .unwrap()
                }
            }
        }
        let user: Option<User> = match &self.get_state {
            FetchState::NotFetching => None,
            FetchState::Fetching => None,
            FetchState::Failed(err) => {
                web_sys::window()
                    .unwrap()
                    .alert_with_message(
                        format!("Get request failed; Reason: {}", err.to_string()).as_str(),
                    )
                    .unwrap();
                None
            }
            FetchState::Success(res) => {
                // web_sys::console::log_1(&format!("{:?}", res).into());
                Some(res.clone())
            }
        };
        html! {
            <div>
                <div>
                    <h1>{ "User stuff" }</h1>
                    <div>
                        <button onclick={ctx.link().callback(|_| Msg::PostUser)}>{ "Create user" }</button>
                        <input
                            type="text"
                            placeholder="Name"
                            ref={self.refs.get("username").unwrap()}
                        />
                        <input
                            type="color"
                            ref={self.refs.get("color").unwrap()}
                        />
                    </div>
                    <div>
                        <button onclick={ctx.link().callback(|_| Msg::GetUser)}>{ "Get user" }</button>
                        <input
                            type="number"
                            placeholder="id"
                            ref={self.refs.get("get-id").unwrap()}
                        />
                        <UserView {user}/>
                    </div>
                    <div>
                        <button onclick={ctx.link().callback(|_| Msg::DeleteUser)}>{ "Delete user" }</button>
                        <input
                            type="number"
                            placeholder="id"
                            ref={self.refs.get("delete-id").unwrap()}
                        />
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

#[derive(Properties, PartialEq)]
struct UserProps {
    user: Option<User>,
}

#[function_component]
fn UserView(props: &UserProps) -> Html {
    match &props.user {
        None => html!(),
        Some(user) => {
            html! {
                <text
                    style={format!("color: rgb({}, {}, {}); margin: .25rem;",
                                   user.color.r, user.color.g, user.color.b)}
                >{ &user.name }</text>
            }
        }
    }
}
