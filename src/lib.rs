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
                    "gaming1",
                    "https://i.gyazo.com/94297d4712add817da9ec7ecda9fa5a4.mp4",
                ),
                ProjectContent::new(
                    ContentType::Video,
                    "gaming2",
                    "https://i.gyazo.com/64c664823fe64229eadfb1cb6a8a461c.mp4",
                ),
            ],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <div>
                { for self.contents.iter().map(|props|
                    html! {
                    <VideoContentCard alt_text=props.alt_text.to_owned() video_url=props.src_url.to_owned(), width=props.width/>
                }) }
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainProject>::new().mount_to_body();
}
