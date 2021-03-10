mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{H1, H5};

pub struct SliderDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SliderDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SliderDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class=classes!("docs-title")>{"Slider"}</H1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <SliderProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        />
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    SliderProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
            </div>
        }
    }
}
