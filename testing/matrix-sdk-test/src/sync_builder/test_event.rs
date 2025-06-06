use ruma::{
    events::{
        presence::PresenceEvent, AnyGlobalAccountDataEvent, AnyRoomAccountDataEvent,
        AnyStrippedStateEvent, AnySyncStateEvent,
    },
    serde::Raw,
};
use serde_json::{from_value as from_json_value, Value as JsonValue};

use crate::test_json;

/// Test events that can be added to the state.
pub enum StateTestEvent {
    Alias,
    Aliases,
    Create,
    Encryption,
    HistoryVisibility,
    JoinRules,
    Member,
    MemberAdditional,
    MemberBan,
    MemberInvite,
    MemberLeave,
    MemberNameChange,
    PowerLevels,
    RedactedInvalid,
    RedactedState,
    RoomAvatar,
    RoomName,
    RoomPinnedEvents,
    RoomTopic,
    Custom(JsonValue),
}

impl StateTestEvent {
    /// Get the JSON representation of this test event.
    pub fn into_json_value(self) -> JsonValue {
        match self {
            Self::Alias => test_json::sync_events::ALIAS.to_owned(),
            Self::Aliases => test_json::sync_events::ALIASES.to_owned(),
            Self::Create => test_json::sync_events::CREATE.to_owned(),
            Self::Encryption => test_json::sync_events::ENCRYPTION.to_owned(),
            Self::HistoryVisibility => test_json::sync_events::HISTORY_VISIBILITY.to_owned(),
            Self::JoinRules => test_json::sync_events::JOIN_RULES.to_owned(),
            Self::Member => test_json::sync_events::MEMBER.to_owned(),
            Self::MemberAdditional => test_json::sync_events::MEMBER_ADDITIONAL.to_owned(),
            Self::MemberBan => test_json::sync_events::MEMBER_BAN.to_owned(),
            Self::MemberInvite => test_json::sync_events::MEMBER_INVITE.to_owned(),
            Self::MemberLeave => test_json::sync_events::MEMBER_LEAVE.to_owned(),
            Self::MemberNameChange => test_json::sync_events::MEMBER_NAME_CHANGE.to_owned(),
            Self::PowerLevels => test_json::sync_events::POWER_LEVELS.to_owned(),
            Self::RedactedInvalid => test_json::sync_events::REDACTED_INVALID.to_owned(),
            Self::RedactedState => test_json::sync_events::REDACTED_STATE.to_owned(),
            Self::RoomAvatar => test_json::sync_events::ROOM_AVATAR.to_owned(),
            Self::RoomName => test_json::sync_events::NAME.to_owned(),
            Self::RoomPinnedEvents => test_json::sync_events::PINNED_EVENTS.to_owned(),
            Self::RoomTopic => test_json::sync_events::TOPIC.to_owned(),
            Self::Custom(json) => json,
        }
    }

    /// Get the typed JSON representation of this test event.
    pub fn into_raw_event(self) -> Raw<AnySyncStateEvent> {
        from_json_value(self.into_json_value()).unwrap()
    }
}

/// Test events that can be added to the stripped state.
pub enum StrippedStateTestEvent {
    Member,
    RoomName,
    Custom(JsonValue),
}

impl StrippedStateTestEvent {
    /// Get the JSON representation of this test event.
    pub fn into_json_value(self) -> JsonValue {
        match self {
            Self::Member => test_json::sync_events::MEMBER_STRIPPED.to_owned(),
            Self::RoomName => test_json::sync_events::NAME_STRIPPED.to_owned(),
            Self::Custom(json) => json,
        }
    }

    /// Get the typed JSON representation of this test event.
    pub fn into_raw_event(self) -> Raw<AnyStrippedStateEvent> {
        from_json_value(self.into_json_value()).unwrap()
    }
}

/// Test events that can be added to the room account data.
pub enum RoomAccountDataTestEvent {
    FullyRead,
    Tags,
    Custom(JsonValue),
}

impl RoomAccountDataTestEvent {
    /// Get the JSON representation of this test event.
    pub fn into_json_value(self) -> JsonValue {
        match self {
            Self::FullyRead => test_json::sync_events::FULLY_READ.to_owned(),
            Self::Tags => test_json::sync_events::TAG.to_owned(),
            Self::Custom(json) => json,
        }
    }

    /// Get the typed JSON representation of this test event.
    pub fn into_raw_event(self) -> Raw<AnyRoomAccountDataEvent> {
        from_json_value(self.into_json_value()).unwrap()
    }
}

/// Test events that can be added to the presence events.
pub enum PresenceTestEvent {
    Presence,
    Custom(JsonValue),
}

impl PresenceTestEvent {
    /// Get the JSON representation of this test event.
    pub fn into_json_value(self) -> JsonValue {
        match self {
            Self::Presence => test_json::sync_events::PRESENCE.to_owned(),
            Self::Custom(json) => json,
        }
    }

    /// Get the typed JSON representation of this test event.
    pub fn into_raw_event(self) -> Raw<PresenceEvent> {
        from_json_value(self.into_json_value()).unwrap()
    }
}

/// Test events that can be added to the global account data.
pub enum GlobalAccountDataTestEvent {
    Direct,
    PushRules,
    Custom(JsonValue),
}

impl GlobalAccountDataTestEvent {
    /// Get the JSON representation of this test event.
    pub fn into_json_value(self) -> JsonValue {
        match self {
            Self::Direct => test_json::sync_events::DIRECT.to_owned(),
            Self::PushRules => test_json::sync_events::PUSH_RULES.to_owned(),
            Self::Custom(json) => json,
        }
    }

    /// Get the typed JSON representation of this test event.
    pub fn into_raw_event(self) -> Raw<AnyGlobalAccountDataEvent> {
        from_json_value(self.into_json_value()).unwrap()
    }
}
