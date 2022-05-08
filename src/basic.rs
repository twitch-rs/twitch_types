/// A user ID.
#[aliri_braid::braid(serde)]
pub struct UserId;

/// A users display name
#[aliri_braid::braid(serde)]
pub struct DisplayName;

/// A nickname, not capitalized.
#[aliri_braid::braid(serde)]
pub struct Nickname;


/// A username, also specified as login. Should not be capitalized.
pub type UserName = Nickname;

/// A reference to a borrowed [`UserName`], also specified as login. Should not be capitalized.
pub type UserNameRef = NicknameRef;
