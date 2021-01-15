use yew::prelude::*;

pub struct SiteFooter(SiteFooterProps);

#[derive(Properties, Clone)]
pub struct SiteFooterProps {}

impl Component for SiteFooter {
    type Message = ();
    type Properties = SiteFooterProps;
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
            <nav class="navbar fixed-bottom navbar-dark bg-dark">
                <div class="container-fluid">
                    <div class="icon-link-container">
                        <a class="icon-link" href="https://twitter.com/sundowns_" target="_blank">
                            <i class="bi bi-twitter"></i>
                        </a>
                        <a class="icon-link" href="https://sundowns.itch.io" target="_blank">
                            <i class="bi bi-controller"></i>
                        </a>
                        <a class="icon-link" href="https://github.com/sundowns" target="_blank">
                            <i class="bi bi-github"></i>
                        </a>
                        <a class="icon-link" href="" target="_blank">
                            <i class="bi bi-discord"></i>
                        </a>
                    </div>
                    <div class="nav-identity">
                        <a class="navbar-text" href="https://twitter.com/sundowns_" target="_blank">
                            {"Tom Smallridge (Sundowns)"}
                        </a>
                        <div class="nav-avatar">
                            <a href="https://twitter.com/sundowns_" target="_blank">
                                <img src="https://avatars0.githubusercontent.com/u/6986083?s=460&u=6ab93fd8d22409a58c4ca18c11b7393a795732e3&v=4"/>
                            </a>
                        </div>
                    </div>
                </div>
            </nav>

        }
    }
}
