use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ContentType {
    Video,
    Screenshot,
}

#[derive(Clone)]
pub struct ProjectContent {
    pub content_type: ContentType,
    pub alt_text: String,
    pub src_url: String,
    pub width: i32,
}

impl ProjectContent {
    pub fn new(content_type: ContentType, alt_text: &str, url: &str) -> ProjectContent {
        ProjectContent {
            content_type: content_type,
            alt_text: alt_text.to_owned(),
            src_url: url.to_owned(),
            width: 640,
        }
    }
}

pub struct VideoContentCard(VideoContentCardProps);

#[derive(Properties, Clone)]
pub struct VideoContentCardProps {
    pub alt_text: String,
    pub video_url: String,
    pub width: i32,
}

impl Component for VideoContentCard {
    type Message = ();
    type Properties = VideoContentCardProps;
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
            <div class="content-box video">
                <video alt={self.0.alt_text.to_owned()} width={self.0.width} autoplay=true muted=true loop=true playsinline=true controls=true><source src={self.0.video_url.to_owned()} type="video/mp4" /></video>
            </div>
        }
    }
}

pub struct ImageContentCard(ImageContentCardProps);

#[derive(Properties, Clone)]
pub struct ImageContentCardProps {
    pub alt_text: String,
    pub image_url: String,
    pub width: i32,
}

impl Component for ImageContentCard {
    type Message = ();
    type Properties = ImageContentCardProps;
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
            <div class="content-box screenshot">
                <img src={self.0.image_url.to_owned()} alt={self.0.alt_text.to_owned()} width={self.0.width} />
            </div>
        }
    }
}
