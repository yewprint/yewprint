mod example;

use crate::ExampleContainer;
use example::*;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewprint::{HtmlSelect, Icon, IconName, InputGroup, Intent, Slider, Text, H1, H5};

pub struct IconDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
    search_string: String,
}

#[derive(Clone)]
pub enum IconDocMsg {
    Example(ExampleProps),
    SearchIcon(String),
}

static ICON_LIST: Lazy<Vec<(String, IconName)>> = Lazy::new(|| {
    IconName::ALL
        .iter()
        .map(|x| (format!("{:?}", x).to_lowercase(), *x))
        .collect::<Vec<_>>()
});

fn get_icon_from_name(name: &str) -> Option<IconName> {
    for (icon_name, icon) in ICON_LIST.iter() {
        if name == icon_name {
            return Some(*icon);
        }
    }

    None
}

impl Component for IconDoc {
    type Message = IconDocMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        IconDoc {
            callback: ctx.link().callback(|x| IconDocMsg::Example(x)),
            state: ExampleProps {
                icon_name: IconName::Print,
                intent: None,
                icon_size: 16,
            },
            search_string: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            IconDocMsg::Example(x) => self.state = x,
            IconDocMsg::SearchIcon(x) => self.search_string = x,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        let search_string = self.search_string.to_lowercase();
        let icon_list = ICON_LIST
            .iter()
            .filter_map(|(name, icon)| {
                name.contains(&search_string).then(|| {
                    html! {
                        <div class={classes!("docs-icon-list-item")}>
                            <Icon
                                icon={*icon}
                                icon_size=20
                            />
                            <Text>{format!("{:?}", icon)}</Text>
                        </div>
                    }
                })
            })
            .collect::<Html>();

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Icon"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    {source}
                    props={Some(html! {
                        <IconProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        />
                    })}
                >
                        <Example ..example_props />
                </ExampleContainer>
                <div class={classes!("docs-icon-search")}>
                    <InputGroup
                        large=true
                        fill=true
                        round=true
                        left_icon={IconName::Search}
                        placeholder="Search for icons..."
                        value={self.search_string.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<HtmlInputElement>().value();
                            IconDocMsg::SearchIcon(value)
                        })}
                    />
                </div>
                <div class={classes!("docs-icon-list")}>
                    {icon_list}
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    IconProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            let option_labels = (0..=100)
            .step_by(1)
            .map(|x| (x, (x % 20 == 0).then(|| format!("{}", x).into())))
            .collect::<Vec<_>>();

            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <p>{"Icon:"}</p>
                        <input
                            class="bp3-input"
                            onchange={self.update_props(ctx, |props, e: Event| {
                                let icon = e.target_dyn_into::<HtmlInputElement>()
                                    .map(|x| x.value().to_lowercase())
                                    .as_deref()
                                    .and_then(|x| get_icon_from_name(x));

                                ExampleProps {
                                    icon_name: icon.unwrap_or_default(),
                                    ..props
                                }
                            })}
                            type="text"
                            value={format!("{:?}", ctx.props().example_props.icon_name)}
                        />
                        <p
                            style="margin-top: 5px;"
                        >
                            {"Select intent:"}
                        </p>
                        <HtmlSelect<Option<Intent>>
                            options={vec![
                                (None, "None".to_string()),
                                (Some(Intent::Primary), "Primary".to_string()),
                                (Some(Intent::Success), "Success".to_string()),
                                (Some(Intent::Warning), "Warning".to_string()),
                                (Some(Intent::Danger), "Danger".to_string()),
                            ]}
                            value={self.example_props.intent}
                            onchange={self.update_props(ctx, |props, intent| ExampleProps {
                                intent,
                                ..props
                            })}
                        />
                        <p
                            style="margin-top: 5px;"
                        >
                            {"Select icon size"}
                        </p>
                        <Slider<i32>
                            selected={ctx.props().example_props.icon_size}
                            values={option_labels}
                            onchange={self.update_props(ctx, |props, icon_size| ExampleProps {
                                icon_size,
                                ..props
                            })}
                            value_label={
                                Cow::Owned(format!("{}", ctx.props().example_props.icon_size))
                            }
                        />
                    </div>
                </div>
            }
        }
}

crate::build_source_code_component!();
