use yew::prelude::*;

use crate::{
    components::{
        social_media_link::SocialMediaLink,
        icons::linkedin::LinkedIn,
    },
};

#[derive(Properties, Clone)]
pub struct LinkedInLinkProps {
    pub username: String,
}

pub struct LinkedInLink {
    props: LinkedInLinkProps,
}

impl Component for LinkedInLink {
    type Message = ();
    type Properties = LinkedInLinkProps;

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
                href={format!("https://www.linkedin.com/in/{}/", self.props.username)}
                label="linkedin"
            >
                <LinkedIn />
            </SocialMediaLink>
        }
    }
}
