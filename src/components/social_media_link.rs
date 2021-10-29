use yew::prelude::*;
use url::Url;

use crate::{
    components::{
        icons::{
            github::GitHub,
            linkedin::LinkedIn,
        },
    },
};

#[derive(Properties, Clone)]
pub struct SocialMediaLinkProps {
    pub href: String,
}

pub struct SocialMediaLink {
    props: SocialMediaLinkProps,
}

impl Component for SocialMediaLink {
    type Message = ();
    type Properties = SocialMediaLinkProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <a
                href={self.props.href.clone()}
                target="_blank"
                rel="noopener noreferrer"
                aria-label="linkedin"
            >
                {self.icon_view()}
            </a>
        }
    }
}

impl SocialMediaLink {
    fn icon_view(&self) -> Html {
        let url = Url::parse(&self.props.href).unwrap();
        match url.domain() {
            Some("www.github.com") => {
                return html! {
                    <GitHub />
                };
            }
            Some("www.linkedin.com") => {
                return html! {
                    <LinkedIn />
                };
            }
            Some(domain) => {
                return html! {
                    <div class=classes!("text-lg", "hover:underline")>
                        {domain}
                    </div>
                }
            }
            None => {
                panic!("No domain found")
            }
        }
    }
}
