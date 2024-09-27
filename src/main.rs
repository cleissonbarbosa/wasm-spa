use yew::prelude::*;
use reqwasm::http::Request;
use serde::Deserialize;

struct App {
    posts: Vec<Post>,
    loading: bool,
}

enum Msg {
    FetchPosts,
    ReceiveResponse(Result<Vec<Post>, reqwasm::Error>),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchPosts);
        App {
            posts: vec![],
            loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchPosts => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get("https://jsonplaceholder.typicode.com/posts")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<Post>>()
                        .await;
                    link.send_message(Msg::ReceiveResponse(response));
                });
                false
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(posts) => {
                        self.posts = posts;
                    }
                    Err(_) => {
                        web_sys::window()
                            .unwrap()
                            .alert_with_message("Erro ao carregar posts")
                            .unwrap();
                    }
                }
                self.loading = false;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.loading {
            html! { <p>{ "Carregando..." }</p> }
        } else {
            html! {
                <div>
                    <h1>{ "Lista de Posts" }</h1>
                    <ul>
                        { for self.posts.iter().map(|post| self.view_post(post)) }
                    </ul>
                </div>
            }
        }
    }
}

impl App {
    fn view_post(&self, post: &Post) -> Html {
        html! {
            <li key={post.id}>
                <h3>{ &post.title }</h3>
                <p>{ &post.body }</p>
            </li>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}