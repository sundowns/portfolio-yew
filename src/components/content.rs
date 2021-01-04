pub enum ContentType {
    Video,
    Screenshot,
}
pub struct ProjectContent {
    content_type: ContentType,
    alt_text: String,
    url: String,
}

impl ProjectContent {
    // Another static method, taking two arguments:
    fn new(content_type: ContentType, alt_text: &str, url: &str) -> ProjectContent {
        ProjectContent {
            content_type: content_type,
            alt_text: alt_text.to_owned(),
            url: url.to_owned(),
        }
    }
}

pub struct VideoContentCard {
    link: ComponentLink<Self>,
}
impl Component for VideoContentCard {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
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
                <a href="https://gyazo.com/94297d4712add817da9ec7ecda9fa5a4"><video alt="Video from Gyazo" width="640" autoplay=true muted=true loop=true playsinline=true controls=true><source src="https://i.gyazo.com/94297d4712add817da9ec7ecda9fa5a4.mp4" type="video/mp4" /></video></a>
            </div>
        }
    }
}
