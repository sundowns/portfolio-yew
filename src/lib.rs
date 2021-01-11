#![recursion_limit = "256"]
mod components;
mod pages;

use components::header::*;
use pages::portfolio::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct MainProject {}

enum Msg {}

impl Component for MainProject {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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
                <SiteHeader/>
                <Portfolio/>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MainProject>::new().mount_to_body();
}
