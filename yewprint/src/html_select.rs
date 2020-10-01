use crate::{ConditionalClass, Icon, IconName}
use yew::prelude::*;

pub struct Html_select {
    props: props,
}

pub struct Props {
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub disable: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    pub children: html::Children
    #[prop_or_default]
    // onChange
    //options
}