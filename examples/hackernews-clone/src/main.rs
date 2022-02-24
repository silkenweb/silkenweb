use std::{fmt::Display, time::Duration};

use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use futures_signals::signal::{Mutable, SignalExt};
use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Deserialize};
use silkenweb::{
    clone,
    elements::html::{self, a, div, h2, header, li, nav, ol, p, span, ul, Div, Li, A},
    macros::{Element, ElementBuilder},
    mount,
    prelude::{ElementEvents, ParentBuilder},
    router::{self, Url},
    task::spawn_local,
};
use timeago::Formatter;

// TODO: Styling
// TODO: Tidy code

#[derive(Clone)]
struct App(Mutable<Option<Content>>);

impl App {
    fn new() -> Self {
        Self(Mutable::new(None))
    }

    fn set_loading(&self) {
        self.0.set(None)
    }

    fn set_content(&self, content: Content) {
        self.0.set(Some(content))
    }

    fn render(&self) -> Div {
        div()
            .child(header().child(nav().children([
                local_link("top", "/topstories"),
                local_link("new", "/newstories"),
                local_link("ask", "/askstories"),
                local_link("show", "/showstories"),
            ])))
            .child(html::main().child_signal(self.0.signal_ref(|content| {
                if let Some(content) = content {
                    content.render()
                } else {
                    p().text("Loading...").into()
                }
            })))
            .build()
    }
}

enum Content {
    FrontPage(Vec<Story>),
    Story(StoryDetail),
    User(UserDetails),
    Unknown,
    Error(String),
}

impl Content {
    async fn load_frontpage(story_type: &str) -> Self {
        Self::ok_or(Self::try_load_frontpage(story_type).await)
    }

    async fn load_story(id: &str) -> Self {
        Self::ok_or(Self::try_load_story(id).await)
    }

    async fn load_user(id: &str) -> Self {
        Self::ok_or(Self::try_load_user(id).await)
    }

    async fn try_load_frontpage(story_type: &str) -> Result<Self, reqwasm::Error> {
        let top_stories: Vec<u64> = query(story_type).await?;

        let stories = top_stories.into_iter().take(STORY_COUNT).map(query_item);

        Ok(Self::FrontPage(
            join_all(stories)
                .await
                .into_iter()
                .filter_map(Result::ok)
                .collect(),
        ))
    }

    async fn try_load_story(id: &str) -> Result<Self, reqwasm::Error> {
        let story: Story = query_item(id).await?;
        let comments = CommentTree::load_vec(&story.kids, 3).await;

        Ok(Self::Story(StoryDetail { story, comments }))
    }

    async fn try_load_user(id: &str) -> Result<Self, reqwasm::Error> {
        let user = query_user(id).await?;
        let submitted = join_all(user.submitted.iter().take(STORY_COUNT).map(query_item))
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        Ok(Self::User(UserDetails { user, submitted }))
    }

    fn ok_or(result: Result<Self, reqwasm::Error>) -> Self {
        match result {
            Ok(ok) => ok,
            Err(err) => Self::Error(err.to_string()),
        }
    }

    fn render(&self) -> Element {
        match self {
            Content::FrontPage(articles) => ol()
                .children(articles.iter().map(|article| li().child(article.render())))
                .into(),
            Content::Story(story) => story.render().into(),
            Content::User(user) => user.render().into(),
            Content::Unknown => p().text("Unknown").into(),
            Content::Error(err) => p().text(err).into(),
        }
    }
}

#[derive(Deserialize)]
struct Story {
    id: u64,
    title: String,
    #[serde(default)]
    score: u64,
    #[serde(default)]
    by: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    descendants: u64,
    #[serde(default)]
    text: String,
    #[serde(default)]
    kids: Vec<u64>,
    url: Option<String>,
}

impl Story {
    fn render(&self) -> Div {
        let score = self.score;
        let descendants = self.descendants;
        let time_ago = time_ago(self.time);
        let id = self.id;
        let url_path = format!("/item/{}", id);

        div()
            .child(h2().child(a().href(self.url.as_ref()).text(&self.title)))
            .child(
                span()
                    .child(span().text(&format!("{score} point{} by ", plural(score))))
                    .child(user_link(&self.by))
                    .child(span().text(&format!(" {time_ago} | ")))
                    .child(
                        a().href(&url_path)
                            .text(&format!("{descendants} comment{}", plural(descendants)))
                            .on_click(move |ev, _| {
                                ev.prevent_default();
                                router::set_url_path(&url_path)
                            }),
                    ),
            )
            .build()
    }
}

#[derive(Deserialize)]
struct Comment {
    #[serde(default)]
    by: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    #[serde(default)]
    text: String,
    #[serde(default)]
    kids: Vec<u64>,
}

#[derive(Deserialize)]
struct User {
    id: String,
    #[serde(default)]
    about: String,
    #[serde(default)]
    karma: u64,
    #[serde(default)]
    submitted: Vec<u64>,
}

struct UserDetails {
    user: User,
    submitted: Vec<Story>,
}

impl UserDetails {
    fn render(&self) -> Div {
        div()
            .child(h2().text(&self.user.id))
            .child(div().effect({
                let about = self.user.about.clone();
                move |elem| elem.set_inner_html(&about)
            }))
            .child(span().text(&format!("{} karma", self.user.karma)))
            .child(ol().children(self.submitted.iter().map(Story::render)))
            .build()
    }
}

struct StoryDetail {
    story: Story,
    comments: Vec<CommentTree>,
}

impl StoryDetail {
    fn render(&self) -> Div {
        div()
            .child(self.story.render())
            .child(div().effect({
                let text = self.story.text.clone();
                move |elem| elem.set_inner_html(&text)
            }))
            .child(ul().children(self.comments.iter().map(|comment| comment.render(0))))
            .build()
    }
}

struct CommentTree {
    comment: Comment,
    children: Vec<CommentTree>,
}

impl CommentTree {
    #[async_recursion(?Send)]
    async fn load_vec(ids: &[u64], depth: usize) -> Vec<Self> {
        join_all(ids.iter().copied().map(|id| Self::load(id, depth)))
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect()
    }

    #[async_recursion(?Send)]
    async fn load(id: u64, depth: usize) -> Result<Self, reqwasm::Error> {
        let comment: Comment = query_item(id).await?;
        let children = if depth > 0 {
            Self::load_vec(&comment.kids, depth - 1).await
        } else {
            Vec::new()
        };

        Ok(Self { comment, children })
    }

    fn render(&self, depth: usize) -> Li {
        let time_ago = time_ago(self.comment.time);
        li().child(user_link(&self.comment.by))
            .child(span().text(&format!(" {time_ago}")))
            .child(div().effect({
                let text = self.comment.text.clone();
                move |elem| elem.set_inner_html(&text)
            }))
            .child(
                ul().children(
                    self.children
                        .iter()
                        .map(move |child| child.render(depth + 1)),
                ),
            )
            .build()
    }
}

fn local_link(name: &str, path: impl Into<String>) -> A {
    let name = name.to_owned();
    let path = path.into();
    a().href(&path)
        .text(&name)
        .on_click(move |ev, _| {
            ev.prevent_default();
            router::set_url_path(&path)
        })
        .build()
}

fn user_link(user: &str) -> A {
    local_link(user, format!("/user/{}", user))
}

async fn query_item<T: DeserializeOwned>(id: impl Display) -> Result<T, reqwasm::Error> {
    query(&format!("item/{id}")).await
}

async fn query_user(id: impl Display) -> Result<User, reqwasm::Error> {
    query(&format!("user/{id}")).await
}

async fn query<T: DeserializeOwned>(path: &str) -> Result<T, reqwasm::Error> {
    Request::get(&format!(
        "https://hacker-news.firebaseio.com/v0/{path}.json"
    ))
    .send()
    .await?
    .json()
    .await
}

fn time_ago(time: DateTime<Utc>) -> String {
    Formatter::new()
        .num_items(1)
        .convert((Utc::now() - time).to_std().unwrap_or(Duration::ZERO))
}

fn plural(count: u64) -> &'static str {
    if count == 1 {
        ""
    } else {
        "s"
    }
}

fn main() {
    let app = App::new();

    spawn_local(router::url().signal_ref(Url::pathname).for_each({
        clone!(app);
        move |pathname| {
            clone!(app);
            async move {
                app.set_loading();

                app.set_content(match pathname.as_str() {
                    "/" => Content::load_frontpage("topstories").await,
                    "/topstories" | "/newstories" | "/askstories" | "/showstories" => {
                        Content::load_frontpage(&pathname).await
                    }
                    item => match *item.split(['/']).collect::<Vec<_>>() {
                        ["", "item", id] => Content::load_story(id).await,
                        ["", "user", id] => Content::load_user(id).await,
                        _ => Content::Unknown,
                    },
                })
            }
        }
    }));

    mount("app", app.render());
}

const STORY_COUNT: usize = 30;
