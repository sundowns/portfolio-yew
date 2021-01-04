use content::{ContentType, ProjectContent};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct MainProject {
    link: ComponentLink<Self>,
    content: [ProjectContent; 4],
}

enum Msg {}

impl Component for MainProject {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            screenshots: [ProjectContent::new(
                ContentType::Video,
                "gaming1",
                "https://i.gyazo.com/94297d4712add817da9ec7ecda9fa5a4.mp4",
            )],
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
                // <ul>
                //     <VideoContentCard/>
                // </ul>
                // TODO: use the screenshots field (iterate over it and display videos)
                <a href="https://gyazo.com/94297d4712add817da9ec7ecda9fa5a4"><video alt="Video from Gyazo" width="640" autoplay=true muted=true loop=true playsinline=true controls=true><source src="https://i.gyazo.com/94297d4712add817da9ec7ecda9fa5a4.mp4" type="video/mp4" /></video></a>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainProject>::new().mount_to_body();
}
