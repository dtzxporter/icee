mod button;

use std::collections::BTreeMap;

use iced::Background;
use serde::Deserialize;
use serde::Serialize;

use iced::Color;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(untagged)]
pub enum RuleLength {
    Float(f32),
    UnsignedInteger(u32),
    String(String),
    #[default]
    None,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuleState {
    Active,
    #[default]
    Any,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Rule {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub widget: Option<String>,
    #[serde(default)]
    pub state: RuleState,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub background_color: Option<String>,
    #[serde(default)]
    pub border_radius: RuleLength,
    #[serde(default)]
    pub width: RuleLength,
    #[serde(default)]
    pub height: String,
}

#[derive(Debug)]
pub struct Rules {
    pub rules: BTreeMap<RuleState, Rule>,
}

impl Rules {
    pub fn any(&self) -> &Rule {
        self.rules
            .get(&RuleState::Any)
            .expect("Failed to find any rule!")
    }

    pub fn active(&self) -> Option<&Rule> {
        self.rules.get(&RuleState::Active)
    }
}

impl Rule {
    pub fn width(&self) -> iced::Length {
        match &self.width {
            RuleLength::UnsignedInteger(integer) => iced::Length::Fixed(*integer as f32),
            RuleLength::Float(float) => iced::Length::Fixed(*float),
            RuleLength::String(string) => iced::Length::Fixed(string.parse().unwrap()),
            RuleLength::None => iced::Length::Shrink,
        }
    }

    pub fn height(&self) -> iced::Length {
        if self.height.is_empty() {
            iced::Length::Shrink
        } else {
            iced::Length::Fixed(self.height.parse().unwrap())
        }
    }

    pub fn merge(&mut self, rule: &Rule) {
        self.color = rule.color.clone().or(self.color.take());
        self.background_color = rule.background_color.clone();
        self.width = rule.width.clone();
        self.height = rule.height.clone();
    }
}

pub(super) fn parse_color(color: &str) -> Option<Color> {
    if color.is_empty() {
        return None;
    }

    if let Ok(color) = csscolorparser::parse(color) {
        let (r, g, b, a) = color.to_linear_rgba();

        Some(Color::from_linear_rgba(
            r as f32, g as f32, b as f32, a as f32,
        ))
    } else {
        None
    }
}

pub(super) fn parse_background_color(background_color: &str) -> Option<Background> {
    if background_color.is_empty() {
        return None;
    }

    if let Ok(color) = csscolorparser::parse(background_color) {
        let (r, g, b, a) = color.to_linear_rgba();

        Some(Background::Color(Color::from_linear_rgba(
            r as f32, g as f32, b as f32, a as f32,
        )))
    } else {
        None
    }
}
