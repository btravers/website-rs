use yew::prelude::*;

use crate::{
    components::{
        social_media_link::SocialMediaLink,
        icons::github::GitHub,
    },
};

#[derive(Properties, Clone)]
pub struct GitHubLinkProps {
    pub username: String,
}

pub struct GitHubLink {
    props: GitHubLinkProps,
}

impl Component for GitHubLink {
    type Message = ();
    type Properties = GitHubLinkProps;

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
            <SocialMediaLink
                href={format!("https://www.github.com/{}/", self.props.username)}
                label="github"
            >
                <GitHub />
            </SocialMediaLink>
        }
    }
}
