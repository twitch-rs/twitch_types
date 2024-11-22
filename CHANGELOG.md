# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.8...Unreleased)

## [v0.4.8] - 2024-11-21

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.7...v0.4.8)

- Added `ContentClassificationId::DebatedSocialIssuesAndPolitics`
- Changed `ResubscriptionEmote` to `EmoteOccurrence`
- Deprecated `ResubscriptionEmote` (alias to `EmoteOccurrence`)

## [v0.4.7] - 2024-11-06

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.6...v0.4.7)

- Added `StreamKey` (feature `stream`)

## [v0.4.6] - 2024-10-13

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.5...v0.4.6)

- Added `SharedChatSessionId` and `WhisperId` (feature `chat`)
- Added `EntitlementId`, `BenefitId`, `OrganizationId`, and `EntitlementCampaignId` (feature `entitlements`)
- Added `ConduitId` and `ConduitShardId` (feature `eventsub`)
- Added `ExtensionId` (feature `extensions`)
- Added `UnbanRequestid` (feature `moderation`)
- Added `CclId`, `GuestStarSessionId`, `GuestStarSlotId`, and `StreamMarkerId` (feature `stream`)

## [v0.4.5] - 2024-04-15

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.4...v0.4.5)

## [v0.4.4] - 2023-11-16

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.3...v0.4.4)

- MSRV changed to 1.67.0
- Added `sub::CommunityGiftId` and new `sub` feature

## [v0.4.3] - 2023-09-17

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.2...v0.4.3)

## [v0.4.2] - 2023-07-16

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.1...v0.4.2)

- Added Content Classification Label IDs

## [v0.4.1] - 2023-02-16

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.4.0...v0.4.1)

- Added charity donation id: `CharityDonationId`

## [v0.4.0] - 2023-01-24

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.11...v0.4.0)

- Removed most dependencies, mainly `aliri_braid`. Instead we manually create the same implementations.
- Removed `serde_repr` and `typed-builder` optional dependencies.
- Renamed `IntoCow::to_cow` -> `IntoCow::into_cow`

## [v0.3.11] - 2023-01-11

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.10...v0.3.11)

## [v0.3.10] - 2022-12-04

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.9...v0.3.10)

## [v0.3.9] - 2022-11-27

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.8...v0.3.9)

## [v0.3.8] - 2022-11-27

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.7...v0.3.8)

## [v0.3.7] - 2022-11-27

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.6...v0.3.7)

### Added

- Added `igdb_id` to TwitchCategory
- Added ZeroFrom and Arbitrary impls for most types behind feature flag `zerofrom` and `arbitrary`

## [v0.3.6] - 2022-10-28

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.5...v0.3.6)

### Added

- Added `IntoCow` trait to easily take braids to be converted into `Cow`s

### Changed

- Added `impl From<&'a Owned> for &'a Ref` and `impl<'a> From<&'a Owned> for Cow<'a, Ref>` for all braids

## [v0.3.5] - 2022-10-22

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.4...v0.3.5)

## [v0.3.4] - 2022-10-17

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.3...v0.3.4)

## [v0.3.3] - 2022-10-16

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.2...v0.3.3)

### Added

- Added `NamedUserColor` and `HexColor`

## [v0.3.2] - 2022-10-15

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.1...v0.3.2)

### Added

- Added `subscription_count`, `new_subscription` and `new_subscription_count` to creator goals

## [v0.3.1] - 2022-10-14

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.3.0...v0.3.1)

## [v0.3.0] - 2022-08-27

### Changed

- Move to new org `twitch-rs`

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.2.0...v0.3.0)

### Breaking

- Changed `aliri_braid` to version `0.2.4`, changing the `new` method for most types to take a owned string.
  - `BadgeSetId::new`, `BlockedTermId::new`, `CategoryId::new`, `ChatBadgeId::new`, `CreatorGoalId::new`,
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

- Added `CharityCampaignId`

## [v0.2.0] - 2022-05-08

- Place types behind features to provide more granularity to the selections.

[Commits](https://github.com/twitch-rs/twitch_types/compare/v0.1.0...v0.2.0)

## [v0.1.0] - 2022-05-08

- Initial move from `twitch_api2` to its own crate.

## [End of Changelog]

Changelog starts on v0.1.0
