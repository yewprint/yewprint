use yew::prelude::*;
use yewprint::{Tab, Tabs};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
    selected: Civilization,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub animate: bool,
    pub vertical: bool,
}

impl Component for Example {
    type Message = Civilization;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
            props,
            selected: Civilization::Minoan,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.selected != msg {
            self.selected = msg;
            true
        } else {
            false
        }
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
        html! {
            <div>
                <Tabs<Civilization>
                    animate=self.props.animate
                    vertical=self.props.vertical
                    selected_tab_id=self.selected
                    onchange=self.link.callback(|x| x)
                    tabs=vec![
                        Tab {
                            disabled: false,
                            id: Civilization::Sumer,
                            panel: html! { <>
                                <strong>{"Sumer"}</strong>
                                {" is the earliest known civilization in the historical \
                                region of southern Mesopotamia (now southern Iraq), \
                                emerging during the Chalcolithic and early Bronze Ages \
                                between the sixth and fifth millennium BCE. It is also \
                                one of the first civilizations in the world, along with \
                                Ancient Egypt, Norte Chico, Minoan civilization, Ancient \
                                China, Mesoamerica and the Indus Valley.  Living along \
                                the valleys of the Tigris and Euphrates, Sumerian farmers \
                                grew an abundance of grain and other crops, the surplus \
                                from which enabled them to form urban settlements. \
                                Prehistoric proto-writing dates back before 3000 BCE. The \
                                earliest texts come from the cities of Uruk and Jemdet \
                                Nasr, and date to between c. 3500 and c. 3000 BCE."}
                            </> },
                            title: html!(<strong>{"Sumer"}</strong>),
                            panel_class: None,
                            title_class: None,
                        },
                        Tab {
                            disabled: false,
                            id: Civilization::Minoan,
                            panel: html! { <>
                                {"The "}<strong>{"Minoan civilization"}</strong>
                                {" was a Bronze Age Aegean civilization on the island \
                                of Crete and other Aegean Islands, flourishing from c. \
                                3000 BC to c. 1450 BC until a late period of decline, \
                                finally ending around 1100 BC. It represents the first \
                                advanced civilization in Europe, leaving behind massive \
                                building complexes, tools, artwork, writing systems, and \
                                a massive network of trade. The civilization was \
                                rediscovered at the beginning of the 20th century through \
                                the work of British archaeologist Sir Arthur Evans. The \
                                name \"Minoan\" derives from the mythical King Minos and \
                                was coined by Evans, who identified the site at Knossos \
                                with the labyrinth and the Minotaur.  The Minoan \
                                civilization has been described as the earliest of its \
                                kind in Europe, and historian Will Durant called the \
                                Minoans \"the first link in the European chain\"."}
                            </> },
                            title: html!(<em>{"Minoan"}</em>),
                            panel_class: None,
                            title_class: None,
                        },
                        Tab {
                            disabled: true,
                            id: Civilization::AncientEgypt,
                            panel: html!(),
                            title: html!(<u>{"Ancient Egypt"}</u>),
                            panel_class: None,
                            title_class: None,
                        },
                        Tab {
                            disabled: false,
                            id: Civilization::IndusValley,
                            panel: html! { <>
                                {"The "}<strong>{"Indus Valley Civilisation (IVC)"}</strong>
                                {" was a Bronze Age civilisation in the northwestern \
                                regions of South Asia, lasting from 3300 BCE to 1300 BCE, \
                                and in its mature form from 2600 BCE to 1900 BCE. \
                                Together with ancient Egypt and Mesopotamia, it was one \
                                of three early civilisations of the Near East and South \
                                Asia, and of the three, the most widespread, its sites \
                                spanning an area stretching from northeast Afghanistan, \
                                through much of Pakistan, and into western and \
                                northwestern India.  It flourished in the basins of the \
                                Indus River, which flows through the length of Pakistan, \
                                and along a system of perennial, mostly monsoon-fed, \
                                rivers that once coursed in the vicinity of the seasonal \
                                Ghaggar-Hakra river in northwest India and eastern \
                                Pakistan."}
                            </> },
                            title: html!("Indus Valley"),
                            panel_class: None,
                            title_class: None,
                        },
                    ]
                />
            </div>
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub enum Civilization {
    Sumer,
    Minoan,
    AncientEgypt,
    IndusValley,
}
