use yew::prelude::*;

pub struct SiteHeader(SiteHeaderProps);

#[derive(Properties, Clone)]
pub struct SiteHeaderProps {}

impl Component for SiteHeader {
    type Message = ();
    type Properties = SiteHeaderProps;
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
            <div class="site-header">
                <p>{"I am a header!"}</p>
            </div>
        }
    }
}
