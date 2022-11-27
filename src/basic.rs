/// A user ID.
#[aliri_braid::braid(serde)]
pub struct UserId;

impl_extra!(numeric, UserId, UserIdRef);

/// A users display name
#[aliri_braid::braid(serde)]
pub struct DisplayName;

impl_extra!(DisplayName, DisplayNameRef);

/// A nickname, not capitalized.
#[aliri_braid::braid(serde)]
pub struct Nickname;

impl_extra!(ascii, Nickname, NicknameRef);

/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A reference to a borrowed [`UserName`], also specified as login. Should not be capitalized.
pub type UserNameRef = NicknameRef;
