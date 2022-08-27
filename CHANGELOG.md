# Change Log

<!-- next-header -->

## [Unreleased] - ReleaseDate

[Commits](https://github.com/Emilgardis/twitch_types/compare/v0.2.0...Unreleased)

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

[Commits](https://github.com/Emilgardis/twitch_types/compare/v0.1.0...v0.2.0)

## [v0.1.0] - 2022-05-08

* Initial move from `twitch_api2` to its own crate.

## [End of Changelog]

Changelog starts on v0.1.0