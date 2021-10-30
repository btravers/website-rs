use yew::prelude::*;

use crate::{
    components::{
        section::Section,
        linkedin_link::LinkedInLink,
        github_link::GitHubLink,
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
                    <div class=classes!("inline-grid", "grid-cols-2", "items-center", "gap-5", "my-5")>
                        <LinkedInLink username="btraversfr" />
                        <GitHubLink username="btravers" />
                    </div>
                </div>
            </Section>
        }
    }
}
