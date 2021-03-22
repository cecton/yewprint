use crate::Intent;
use yew::prelude::*;

pub struct Slider {
    props: SliderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub intent: Option<Intent>,
    pub value: i32,
    #[prop_or_default]
    pub step_size: i32,
    #[prop_or_default]
    pub min: i32,
    pub max: i32,
    #[prop_or_default]
    pub onchange: Callback<i32>,
}

impl Component for Slider {
    type Message = ();
    type Properties = SliderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let percentage = (100 * self.props.value / self.props.max).clamp(0, 100);

        html! {
            <div
                class=classes!(
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                    self.props.intent,
                )
            >
                <div class=classes!("bp3-slider-track")>
                    <div class=classes!("bp3-slider-progress")>
                    </div>
                </div>
                <div class=classes!("bp3-slider-axis")>
                    <div
                        class=classes!("bp3-slider-label")
                        style="left: 0%;"
                    >
                        {self.props.min}
                    </div>
                    <div
                        class=classes!("bp3-slider-label")
                        style="left: 100%;"
                    >
                        {self.props.max}
                    </div>
                </div>
                <span
                    class=classes!("bp3-slider-handle")
                    style=format!("left: {}%", percentage)
                >
                    <span class=classes!("bp3-slider-label")>
                        {self.props.value}
                    </span>
                </span>
            </div>
        }
    }
}
