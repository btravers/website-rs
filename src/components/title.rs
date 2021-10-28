use yew::prelude::*;

use crate::{
    components::{
        section::Section,
        social_media_link::SocialMediaLink,
        icons::{
            linkedin::LinkedIn,
            github::GitHub,
        },
    },
};

pub struct Title {}

impl Component for Title {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Section dark=true>
                <div class=classes!("h-full", "flex", "flex-col", "gap-3", "items-center", "justify-center")>
                    <img class=classes!("max-w-4xl") src="images/logo.svg" alt=""/>
                    <div class=classes!("text-xl", "md:text-3xl", "text-center")>
                        <h2>{ "développeur full stack freelance" }</h2>
                    </div>
                    <div class=classes!("inline-grid", "grid-cols-2", "gap-5", "my-5")>
                        <SocialMediaLink href="https://www.linkedin.com/in/btraversfr/">
                            <LinkedIn />
                        </SocialMediaLink>
                        <SocialMediaLink href="https://www.github.com/btravers/">
                            <GitHub />
                        </SocialMediaLink>
                    </div>
                </div>
            </Section>
        }
    }
}
