mod button;

use std::collections::BTreeMap;
use std::sync::Arc;

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
    pub border_radius: Option<RuleLength>,
    #[serde(default)]
    pub width: RuleLength,
    #[serde(default)]
    pub height: RuleLength,
}

#[derive(Debug, Clone)]
pub struct Rules {
    pub rules: Arc<BTreeMap<RuleState, Rule>>,
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
    pub fn width(&self) -> Option<iced::Length> {
        Some(match &self.width {
            RuleLength::UnsignedInteger(integer) => iced::Length::Fixed(*integer as f32),
            RuleLength::Float(float) => iced::Length::Fixed(*float),
            RuleLength::String(string) => iced::Length::Fixed(string.parse().unwrap()),
            RuleLength::None => return None,
        })
    }

    pub fn height(&self) -> Option<iced::Length> {
        Some(match &self.height {
            RuleLength::UnsignedInteger(integer) => iced::Length::Fixed(*integer as f32),
            RuleLength::Float(float) => iced::Length::Fixed(*float),
            RuleLength::String(string) => iced::Length::Fixed(string.parse().unwrap()),
            RuleLength::None => return None,
        })
    }

    pub fn border_radius(&self) -> Option<iced::border::Radius> {
        self.border_radius.as_ref().and_then(|border_radius| {
            Some(match border_radius {
                RuleLength::UnsignedInteger(integer) => iced::border::Radius::from(*integer as f32),
                RuleLength::Float(float) => iced::border::Radius::from(*float),
                RuleLength::String(string) => {
                    iced::border::Radius::from(string.parse::<f32>().unwrap())
                }
                RuleLength::None => return None,
            })
        })
    }

    #[inline(always)]
    pub fn merge(&mut self, rule: &Rule) {
        self.color = rule.color.clone().or(self.color.take());
        self.background_color = rule
            .background_color
            .clone()
            .or(self.background_color.take());
        self.border_radius = rule.border_radius.clone().or(self.border_radius.take());
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
