#![recursion_limit = "256"]
mod components;
mod pages;

use components::content::*;
use components::footer::*;
use pages::game::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct MainProject {}

enum Msg {}

impl Component for MainProject {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
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
            <>
                // Do routing here basically
                <GameProject contents={vec![
                    ProjectContent::new(
                        ContentType::Video,
                        "Weapon Crates",
                        "https://i.gyazo.com/c2544dd6eb1d7e6580cf03cab69b3195.mp4",
                    ),
                    ProjectContent::new(
                        ContentType::Video,
                        "Explosive Barrels",
                        "https://i.gyazo.com/3c269e5d57c6e083df146ccd60db1a0f.mp4",
                    ),
                    ProjectContent::new(
                        ContentType::Video,
                        "Sticky Launcher",
                        "https://i.gyazo.com/35c0dcc1daa1352bf5309834aa491905.mp4",
                    ),
                    ProjectContent::new(
                        ContentType::Screenshot,
                        "screenshot example",
                        "https://i.gyazo.com/affc5d07f2fd5864b5475f93bd057445.png",
                    ),
                ]}/>
                <SiteFooter/>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainProject>::new().mount_to_body();
}
