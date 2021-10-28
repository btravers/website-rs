use yew::prelude::*;
use boolinator::Boolinator;

#[derive(Properties, Clone)]
pub struct SectionProps {
    #[prop_or(false)]
    pub dark: bool,
    #[prop_or_default]
    pub children: Children,
}

pub struct Section {
    props: SectionProps,
}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let SectionProps {
            dark,
            children,
        } = &self.props;
        html! {
            <div class=classes!("min-h-screen", "flex", "flex-stretch", dark.as_some("dark"))>
                <div class=classes!("container", "mx-auto", "px-5")>
                    { for children.iter() }
                </div>
            </div>
        }
    }
}
