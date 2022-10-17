use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A color in hex
#[aliri_braid::braid(serde)]
pub struct HexColor;

/// Colors a user can have
#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(field_identifier, rename_all = "snake_case")]
pub enum NamedUserColor<'a> {
    /// Blue
    Blue,
    /// Blue Violet
    BlueViolet,
    /// Cadet Blue
    CadetBlue,
    /// Chocolate
    Chocolate,
    /// Coral
    Coral,
    /// Dodger Blue
    DodgerBlue,
    /// Firebrick
    Firebrick,
    /// Golden Rod
    GoldenRod,
    /// Green
    Green,
    /// Hot Pink
    HotPink,
    /// Orange Red
    OrangeRed,
    /// Red
    Red,
    /// Sea Green
    SeaGreen,
    /// Spring Green
    SpringGreen,
    /// Yellow Green
    YellowGreen,
    /// A hex color
    #[serde(borrow = "'a")]
    Hex(Cow<'a, HexColorRef>),
}

impl<'a> NamedUserColor<'a> {
    /// Creates a owned [NamedUserColor<'static>](NamedUserColor) from a borrowed [NamedUserColor<'a>](NamedUserColor)
    pub fn to_owned(&self) -> NamedUserColor<'static> {
        match self {
            NamedUserColor::Blue => NamedUserColor::Blue,
            NamedUserColor::BlueViolet => NamedUserColor::BlueViolet,
            NamedUserColor::CadetBlue => NamedUserColor::CadetBlue,
            NamedUserColor::Chocolate => NamedUserColor::Chocolate,
            NamedUserColor::Coral => NamedUserColor::Coral,
            NamedUserColor::DodgerBlue => NamedUserColor::DodgerBlue,
            NamedUserColor::Firebrick => NamedUserColor::Firebrick,
            NamedUserColor::GoldenRod => NamedUserColor::GoldenRod,
            NamedUserColor::Green => NamedUserColor::Green,
            NamedUserColor::HotPink => NamedUserColor::HotPink,
            NamedUserColor::OrangeRed => NamedUserColor::OrangeRed,
            NamedUserColor::Red => NamedUserColor::Red,
            NamedUserColor::SeaGreen => NamedUserColor::SeaGreen,
            NamedUserColor::SpringGreen => NamedUserColor::SpringGreen,
            NamedUserColor::YellowGreen => NamedUserColor::YellowGreen,
            NamedUserColor::Hex(hex) => NamedUserColor::Hex(hex.as_ref().to_owned().into()),
        }
    }

    /// All named colors
    pub fn all() -> &'static [NamedUserColor<'static>] {
        &[
            NamedUserColor::Blue,
            NamedUserColor::BlueViolet,
            NamedUserColor::CadetBlue,
            NamedUserColor::Chocolate,
            NamedUserColor::Coral,
            NamedUserColor::DodgerBlue,
            NamedUserColor::Firebrick,
            NamedUserColor::GoldenRod,
            NamedUserColor::Green,
            NamedUserColor::HotPink,
            NamedUserColor::OrangeRed,
            NamedUserColor::Red,
            NamedUserColor::SeaGreen,
            NamedUserColor::SpringGreen,
            NamedUserColor::YellowGreen,
        ]
    }

    /// Return this color in [hex](HexColor)
    pub fn as_hex(&'a self) -> &'a HexColorRef {
        match self {
            NamedUserColor::Blue => HexColorRef::from_static("#0000FF"),
            NamedUserColor::BlueViolet => HexColorRef::from_static("#8A2BE2"),
            NamedUserColor::CadetBlue => HexColorRef::from_static("#5F9EA0"),
            NamedUserColor::Chocolate => HexColorRef::from_static("#D2691E"),
            NamedUserColor::Coral => HexColorRef::from_static("#FF7F50"),
            NamedUserColor::DodgerBlue => HexColorRef::from_static("#1E90FF"),
            NamedUserColor::Firebrick => HexColorRef::from_static("#B22222"),
            NamedUserColor::GoldenRod => HexColorRef::from_static("#DAA520"),
            NamedUserColor::Green => HexColorRef::from_static("#008000"),
            NamedUserColor::HotPink => HexColorRef::from_static("#FF69B4"),
            NamedUserColor::OrangeRed => HexColorRef::from_static("#FF4500"),
            NamedUserColor::Red => HexColorRef::from_static("#FF0000"),
            NamedUserColor::SeaGreen => HexColorRef::from_static("#2E8B57"),
            NamedUserColor::SpringGreen => HexColorRef::from_static("#00FF7F"),
            NamedUserColor::YellowGreen => HexColorRef::from_static("#ADFF2F"),
            NamedUserColor::Hex(hex) => hex,
        }
    }
}

impl Serialize for NamedUserColor<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            NamedUserColor::Blue => "blue",
            NamedUserColor::BlueViolet => "blue_violet",
            NamedUserColor::CadetBlue => "cadet_blue",
            NamedUserColor::Chocolate => "chocolate",
            NamedUserColor::Coral => "coral",
            NamedUserColor::DodgerBlue => "dodger_blue",
            NamedUserColor::Firebrick => "firebrick",
            NamedUserColor::GoldenRod => "golden_rod",
            NamedUserColor::Green => "green",
            NamedUserColor::HotPink => "hot_pink",
            NamedUserColor::OrangeRed => "orange_red",
            NamedUserColor::Red => "red",
            NamedUserColor::SeaGreen => "sea_green",
            NamedUserColor::SpringGreen => "spring_green",
            NamedUserColor::YellowGreen => "yellow_green",
            NamedUserColor::Hex(o) => o.as_str(),
        })
    }
}

impl<'a> TryFrom<&'a str> for NamedUserColor<'a> {
    type Error = serde::de::value::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use serde::de::IntoDeserializer;

        NamedUserColor::deserialize(s.into_deserializer())
    }
}

impl<'a> From<Cow<'a, HexColorRef>> for NamedUserColor<'a> {
    fn from(color: Cow<'a, HexColorRef>) -> Self { NamedUserColor::Hex(color) }
}

impl<'a> From<HexColor> for NamedUserColor<'a> {
    fn from(color: HexColor) -> Self { NamedUserColor::Hex(color.into()) }
}

impl std::fmt::Display for NamedUserColor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.serialize(f) }
}

#[cfg(test)]
#[test]
fn color() {
    let colors = ["red", "hot_pink", "#9146FF"];
    let check = vec![
        NamedUserColor::Red,
        NamedUserColor::HotPink,
        NamedUserColor::Hex(HexColorRef::from_static("#9146FF").into()),
    ];
    assert_eq!(
        check,
        colors
            .into_iter()
            .map(|c: &str| c.try_into())
            .collect::<Result<Vec<NamedUserColor>, serde::de::value::Error>>()
            .unwrap()
    );
    assert_eq!(
        check.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
        colors
    )
}
