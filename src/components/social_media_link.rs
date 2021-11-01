use yew::prelude::*;

use crate::{
    components::{
        icons::{
            github::GitHub,
            linkedin::LinkedIn,
        },
    },
};

#[derive(Clone)]
pub enum SocialMedia {
    GITHUB,
    LINKEDIN,
}

#[derive(Properties, Clone)]
pub struct SocialMediaLinkProps {
    pub media: SocialMedia,
    pub username: String,
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
                href={self.href()}
                target="_blank"
                rel="noopener noreferrer"
                aria-label={self.label()}
            >
                { self.icon() }
            </a>
        }
    }
}

impl SocialMediaLink {
    fn href(&self) -> String {
        match self.props.media {
            SocialMedia::GITHUB => format!("https://www.github.com/{}/", self.props.username),
            SocialMedia::LINKEDIN => format!("https://www.linkedin.com/in/{}/", self.props.username),
        }
    }

    fn label(&self) -> String {
        match self.props.media {
            SocialMedia::GITHUB => String::from("github"),
            SocialMedia::LINKEDIN => String::from("linkedin"),
        }
    }

    fn icon(&self) -> Html {
        match self.props.media {
            SocialMedia::GITHUB => html! {
                <GitHub />
            },
            SocialMedia::LINKEDIN => html! {
                <LinkedIn />
            },
        }
    }
}


