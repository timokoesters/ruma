//! [POST /_matrix/federation/v1/publicRooms](https://matrix.org/docs/spec/server_server/r0.1.4#post-matrix-federation-v1-publicrooms)

use js_int::UInt;
use ruma_api::ruma_api;
use ruma_common::directory::{
    Filter, IncomingFilter, IncomingRoomNetwork, PublicRoomsChunk, RoomNetwork,
};

ruma_api! {
    metadata: {
        description: "Get the list of rooms in this homeserver's public directory.",
        method: POST,
        name: "get_public_rooms_filtered",
        path: "/_matrix/federation/v1/publicRooms",
        rate_limited: false,
        requires_authentication: true,
    }

    #[derive(Default)]
    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    request: {
        /// Limit for the number of results to return.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<UInt>,

        /// Pagination token from a previous request.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub since: Option<&'a str>,

        /// Filter to apply to the results.
        #[serde(default, skip_serializing_if = "Filter::is_empty")]
        pub filter: Filter<'a>,

        /// Network to fetch the public room lists from.
        #[serde(flatten, skip_serializing_if = "ruma_serde::is_default")]
        pub room_network: RoomNetwork<'a>,
    }

    #[derive(Default)]
    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    response: {
        /// A paginated chunk of public rooms.
        pub chunk: Vec<PublicRoomsChunk>,

        /// A pagination token for the response.
        pub next_batch: Option<String>,

        /// A pagination token that allows fetching previous results.
        pub prev_batch: Option<String>,

        /// An estimate on the total number of public rooms, if the server has an estimate.
        pub total_room_count_estimate: Option<UInt>,
    }
}

impl Request<'_> {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new() -> Self {
        Default::default()
    }
}
