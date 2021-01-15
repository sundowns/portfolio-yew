use crate::components::content::*;
use yew::prelude::*;

pub struct GameProject(GameProjectProps);

#[derive(Properties, Clone)]
pub struct GameProjectProps {
    pub contents: Vec<ProjectContent>,
}

impl Component for GameProject {
    type Message = ();
    type Properties = GameProjectProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 0: props }
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component isn't expected to change so just false for now...
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="page-container">
                <div class="content-list">
                    { for self.0.contents.iter().map(|props|
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
            </div>
        }
    }
}
