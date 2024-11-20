manual_braid! {
    /// A Badge set ID
    pub struct BadgeSetId;
    pub struct BadgeSetIdRef;
}
impl_extra!(BadgeSetId, BadgeSetIdRef);

manual_braid! {
    /// A channel chat badge ID
    pub struct ChatBadgeId;
    pub struct ChatBadgeIdRef;
}
impl_extra!(ChatBadgeId, ChatBadgeIdRef);

manual_braid! {
    /// A chat Emote ID
    pub struct EmoteId;
    pub struct EmoteIdRef;
}
impl_extra!(EmoteId, EmoteIdRef);

impl EmoteIdRef {
    /// Generates url for this emote.
    ///
    /// Generated URL will be `"https://static-cdn.jtvnw.net/emoticons/v2/{emote_id}/default/light/1.0"`
    pub fn default_render(&self) -> String {
        EmoteUrlBuilder {
            id: self.into(),
            animation_setting: None,
            theme_mode: EmoteThemeMode::Light,
            scale: EmoteScale::Size1_0,
            template: EMOTE_V2_URL_TEMPLATE.into(),
        }
        .render()
    }

    /// Create a [`EmoteUrlBuilder`] for this emote
    pub fn url(&self) -> EmoteUrlBuilder<'_> { EmoteUrlBuilder::new(self) }
}

/// Emote url template
pub(crate) static EMOTE_V2_URL_TEMPLATE: &str =
    "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}";

/// Formats for an emote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum EmoteAnimationSetting {
    /// Static
    Static,
    /// Animated
    Animated,
}

impl std::fmt::Display for EmoteAnimationSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EmoteAnimationSetting::Static => "static",
            EmoteAnimationSetting::Animated => "animated",
        })
    }
}

/// Background themes available for an emote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum EmoteThemeMode {
    /// Light
    Light,
    /// Dark
    Dark,
}

impl Default for EmoteThemeMode {
    fn default() -> Self { Self::Light }
}

impl std::fmt::Display for EmoteThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EmoteThemeMode::Light => "light",
            EmoteThemeMode::Dark => "dark",
        })
    }
}

/// Scales available for an emote.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub enum EmoteScale {
    /// 1.0
    #[cfg_attr(feature = "serde", serde(rename = "1.0"))]
    Size1_0,
    /// 2.0
    #[cfg_attr(feature = "serde", serde(rename = "2.0"))]
    Size2_0,
    /// 3.0
    #[cfg_attr(feature = "serde", serde(rename = "3.0"))]
    Size3_0,
}

impl Default for EmoteScale {
    fn default() -> Self { Self::Size1_0 }
}

impl std::fmt::Display for EmoteScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            EmoteScale::Size1_0 => "1.0",
            EmoteScale::Size2_0 => "2.0",
            EmoteScale::Size3_0 => "3.0",
        })
    }
}

/// Builder for [emote URLs](https://dev.twitch.tv/docs/irc/emotes#emote-cdn-url-format).
///
/// # Examples
///
/// ```rust
/// # use twitch_types::EmoteId;
/// let emote_id = EmoteId::from("emotesv2_dc24652ada1e4c84a5e3ceebae4de709");
/// assert_eq!(emote_id.url().size_3x().dark_mode().render(), "https://static-cdn.jtvnw.net/emoticons/v2/emotesv2_dc24652ada1e4c84a5e3ceebae4de709/default/dark/3.0")
/// ```
#[derive(Debug, Clone)]
pub struct EmoteUrlBuilder<'a> {
    pub(crate) id: std::borrow::Cow<'a, EmoteIdRef>,
    pub(crate) animation_setting: Option<EmoteAnimationSetting>,
    pub(crate) theme_mode: EmoteThemeMode,
    pub(crate) scale: EmoteScale,
    pub(crate) template: std::borrow::Cow<'a, str>,
}

impl EmoteUrlBuilder<'_> {
    // FIXME: AsRef
    /// Construct a new [`EmoteUrlBuilder`] from a [`EmoteId`]
    ///
    /// Defaults to `1.0` scale, `default` animation and `light` theme.
    pub fn new(id: &EmoteIdRef) -> EmoteUrlBuilder<'_> {
        EmoteUrlBuilder {
            id: id.into(),
            animation_setting: <_>::default(),
            theme_mode: <_>::default(),
            scale: <_>::default(),
            template: EMOTE_V2_URL_TEMPLATE.into(),
        }
    }

    /// Set size to 1.0
    pub fn size_1x(mut self) -> Self {
        self.scale = EmoteScale::Size1_0;
        self
    }

    /// Set size to 2.0
    pub fn size_2x(mut self) -> Self {
        self.scale = EmoteScale::Size2_0;
        self
    }

    /// Set size to 3.0
    pub fn size_3x(mut self) -> Self {
        self.scale = EmoteScale::Size3_0;
        self
    }

    /// Set theme to dark mode
    pub fn dark_mode(mut self) -> Self {
        self.theme_mode = EmoteThemeMode::Dark;
        self
    }

    /// Set theme to light mode
    pub fn light_mode(mut self) -> Self {
        self.theme_mode = EmoteThemeMode::Light;
        self
    }

    /// Set animation mode to default
    pub fn animation_default(mut self) -> Self {
        self.animation_setting = None;
        self
    }

    /// Set animation mode to static
    pub fn animation_static(mut self) -> Self {
        self.animation_setting = Some(EmoteAnimationSetting::Static);
        self
    }

    /// Set animation mode to animate
    pub fn animation_animated(mut self) -> Self {
        self.animation_setting = Some(EmoteAnimationSetting::Animated);
        self
    }

    /// Create the URL for this emote.
    pub fn render(self) -> String {
        if self.template != "https://static-cdn.jtvnw.net/emoticons/v2/{{id}}/{{format}}/{{theme_mode}}/{{scale}}" {
            let custom_template = |builder: &EmoteUrlBuilder| -> Option<String> {
                let mut template = self.template.clone().into_owned();
                let emote_id_range = template.find("{{id}}")?;
                template.replace_range(emote_id_range..emote_id_range+"{{id}}".len(), builder.id.as_str());
                let format_range = template.find("{{format}}")?;
                template.replace_range(format_range..format_range+"{{format}}".len(), &builder.animation_setting.as_ref().map(|s| s.to_string()).unwrap_or_else(|| String::from("default")));
                let theme_mode_range = template.find("{{theme_mode}}")?;
                template.replace_range(theme_mode_range..theme_mode_range+"{{theme_mode}}".len(), &builder.theme_mode.to_string());
                let scale_range = template.find("{{scale}}")?;
                template.replace_range(scale_range..scale_range+"{{scale}}".len(), &builder.scale.to_string());
                if template.contains("{{") || template.contains("}}") {
                    None
                } else {
                    Some(template)
                }
            };
            if let Some(template) = custom_template(&self) {
                return template
            } else {
                #[cfg(feature = "tracing")]
                tracing::warn!(template = %self.template, "emote builder was supplied an invalid or unknown template url, falling back to standard builder");
            }
        }
        // fallback to known working template
        format!("https://static-cdn.jtvnw.net/emoticons/v2/{emote_id}/{animation_setting}/{theme_mode}/{scale}",
            emote_id = self.id,
            animation_setting = self.animation_setting.as_ref().map(|s| s.to_string()).unwrap_or_else(|| String::from("default")),
            theme_mode = self.theme_mode,
            scale = self.scale,
        )
    }
}

manual_braid! {
    /// An Emote Set ID
    pub struct EmoteSetId;
    pub struct EmoteSetIdRef;
}
impl_extra!(EmoteSetId, EmoteSetIdRef);

/// An emote index as defined by eventsub, similar to IRC `emotes` twitch tag.
#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct EmoteOccurrence {
    /// The index of where the Emote starts in the text.
    pub begin: i64,
    /// The index of where the Emote ends in the text.
    pub end: i64,
    /// The emote ID.
    pub id: EmoteId,
}

impl std::fmt::Display for EmoteOccurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.id, self.begin, self.end)
    }
}

/// An emote index as defined by eventsub, similar to IRC `emotes` twitch tag.
#[deprecated(since = "0.4.8", note = "Use EmoteOccurrence instead")]
pub type ResubscriptionEmote = EmoteOccurrence;

/// Links to the same image of different sizes
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Image {
    /// URL to png of size 28x28
    pub url_1x: String,
    /// URL to png of size 56x56
    pub url_2x: String,
    /// URL to png of size 112x112
    pub url_4x: String,
}
