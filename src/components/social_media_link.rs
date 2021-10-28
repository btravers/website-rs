use yew::{
    prelude::*,
    virtual_dom::VChild,
    html::ChildrenRenderer,
};

use crate::{
    components::{
        icons::{
            github::GitHub,
            linkedin::LinkedIn,
        },
    },
};

#[derive(Clone, derive_more::From)]
pub enum Icon {
    GitHub(VChild<GitHub>),
    LinkedIn(VChild<LinkedIn>),
}

impl Into<Html> for Icon {
    fn into(self) -> Html {
        match self {
            Self::GitHub(child) => child.into(),
            Self::LinkedIn(child) => child.into(),
        }
    }
}

#[derive(Properties, Clone)]
pub struct SocialMediaLinkProps {
    pub href: String,
    pub children: ChildrenRenderer<Icon>,
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
        let SocialMediaLinkProps {
            href,
            children,
        } = &self.props;
        html! {
            <a
                href={href.clone()}
                target="_blank"
                rel="noopener noreferrer"
                aria-label="linkedin"
            >
                { for children.iter() }
            </a>
        }
    }
}
