mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Elevation, Switch, H1, H5, HtmlSelect};

pub struct CardDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for CardDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CardDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                elevation: Some(Elevation::Level0),
                interactive: false,
            },
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
                <H1 class="docs-title">{"Card"}</H1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <CardProps
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
    CardProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })
                            checked=self.props.interactive
                            label="Toggle interaction"
                        />
                        <p>{"Elevation:"}</p>
                        <HtmlSelect<Option<Elevation>>
                            options={vec![
                                (Some(Elevation::Level0), "Level 0".to_string()),
                                (Some(Elevation::Level1), "Level 1".to_string()),
                                (Some(Elevation::Level2), "Level 2".to_string()),
                                (Some(Elevation::Level3), "Level 3".to_string()),
                                (Some(Elevation::Level4), "Level 4".to_string()),
                            ]}
                            onchange=self.update_props(|props, elevation| ExampleProps {
                                elevation,
                                ..props
                            })
                        />
                    </div>
                </div>
            }
        }
}
