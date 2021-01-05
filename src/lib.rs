#![recursion_limit = "256"]
mod components;

use components::content::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct MainProject {
    link: ComponentLink<Self>,
    contents: Vec<ProjectContent>,
}

enum Msg {}

impl Component for MainProject {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            contents: vec![
                ProjectContent::new(
                    ContentType::Video,
                    "Sticky Launcher",
                    "https://i.gyazo.com/c2544dd6eb1d7e6580cf03cab69b3195.mp4",
                ),
                ProjectContent::new(
                    ContentType::Video,
                    "gaming1",
                    "https://i.gyazo.com/3c269e5d57c6e083df146ccd60db1a0f.mp4",
                ),
                ProjectContent::new(
                    ContentType::Video,
                    "gaming2",
                    "https://i.gyazo.com/64c664823fe64229eadfb1cb6a8a461c.mp4",
                ),
                ProjectContent::new(
                    ContentType::Screenshot,
                    "screenshot example",
                    "https://i.gyazo.com/affc5d07f2fd5864b5475f93bd057445.png",
                ),
            ],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="content-list">
                { for self.contents.iter().map(|props|
                    match props.content_type {
                        ContentType::Video => html! {
                            <VideoContentCard alt_text=props.alt_text.to_owned() video_url=props.src_url.to_owned(), width=props.width />
                        },
                        ContentType::Screenshot => html! {
                            <ImageContentCard alt_text=props.alt_text.to_owned(), image_url=props.src_url.to_owned(), width=props.width />
                        },
                    })
                }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainProject>::new().mount_to_body();
}
