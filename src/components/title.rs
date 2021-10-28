use yew::prelude::*;

use super::section::Section;
use super::linkedin::LinkedIn;
use super::github::GitHub;

pub struct Title {}

impl Component for Title {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
                        <a
                            href="https://www.linkedin.com/in/btraversfr/"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="linkedin"
                        >
                            <LinkedIn/>
                        </a>
                        <a
                            href="https://www.github.com/btravers/"
                            target="_blank"
                            rel="noopener noreferrer"
                            aria-label="github"
                        >
                            <GitHub/>
                        </a>
                    </div>
                </div>
            </Section>
        }
    }
}
