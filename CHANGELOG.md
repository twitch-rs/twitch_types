# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.5...Unreleased)

### Changed

* Added `impl From<&'a Owned> for &'a Ref` and `impl<'a> From<&'a Owned> for Cow<'a, Ref>` for all braids

## [v0.3.5] - 2022-10-22

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.4...v0.3.5)

## [v0.3.4] - 2022-10-17

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.3...v0.3.4)

## [v0.3.3] - 2022-10-16

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.2...v0.3.3)

### Added

* Added `NamedUserColor` and `HexColor`

## [v0.3.2] - 2022-10-15

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.1...v0.3.2)

### Added

* Added `subscription_count`, `new_subscription` and `new_subscription_count` to creator goals

## [v0.3.1] - 2022-10-14

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.0...v0.3.1)

## [v0.3.0] - 2022-08-27

### Changed

* Move to new org `twitch-rs`

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.2.0...v0.3.0)

### Breaking

* Changed `aliri_braid` to version `0.2.4`, changing the `new` method for most types to take a owned string.
  * `BadgeSetId::new`, `BlockedTermId::new`, `CategoryId::new`, `ChatBadgeId::new`, `CreatorGoalId::new`,
    `DisplayName::new`, `EmoteId::new`, `EmoteSetId::new`, `EventSubId::new`, `HypeTrainId::new`,
    `MsgId::new`, `PollChoiceId::new`, `PollId::new`, `PredictionId::new`, `PredictionOutcomeId::new`,
    `RedemptionId::new`, `RewardId::new`, `StreamId::new`, `StreamSegmentId::new`, `TagId::new`,
    `TeamId::new`, `Timestamp::new`, `UserId::new`, `VideoId::new`, `BadgeSetId::new`,
    `ChatBadgeId::new`, `EmoteId::new`, `EmoteSetId::new`, `EventSubId::new`, `CreatorGoalId::new`,
    `BlockedTermId::new`, `MsgId::new`, `PollChoiceId::new`, `PollId::new`, `PredictionId::new`,
    `PredictionOutcomeId::new`, `RedemptionId::new`, `RewardId::new`, `CategoryId::new`, `HypeTrainId::new`,
    `StreamId::new`, `StreamSegmentId::new`, `TagId::new`, `TeamId::new`, `VideoId::new`,
    `Timestamp::new` and `Timestamp::new_unchecked` now take a owned `String` instead of `impl Into<String>`

### Added

* Added `CharityCampaignId`

## [v0.2.0] - 2022-05-08

* Place types behind features to provide more granularity to the selections.

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.1.0...v0.2.0)

## [v0.1.0] - 2022-05-08

* Initial move from `twitch_api2` to its own crate.

## [End of Changelog]

Changelog starts on v0.1.0
