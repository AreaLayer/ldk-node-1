// This file is Copyright its original authors, visible in version control history.
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. You may not use this file except in
// accordance with one or both of these licenses.

//! Objects and traits for data persistence.

pub mod sqlite_store;
#[cfg(test)]
pub(crate) mod test_utils;
pub(crate) mod utils;
#[cfg(any(vss, vss_test))]
pub(crate) mod vss_store;

/// The event queue will be persisted under this key.
pub(crate) const EVENT_QUEUE_PERSISTENCE_PRIMARY_NAMESPACE: &str = "";
pub(crate) const EVENT_QUEUE_PERSISTENCE_SECONDARY_NAMESPACE: &str = "";
pub(crate) const EVENT_QUEUE_PERSISTENCE_KEY: &str = "events";

/// The peer information will be persisted under this key.
pub(crate) const PEER_INFO_PERSISTENCE_PRIMARY_NAMESPACE: &str = "";
pub(crate) const PEER_INFO_PERSISTENCE_SECONDARY_NAMESPACE: &str = "";
pub(crate) const PEER_INFO_PERSISTENCE_KEY: &str = "peers";

/// The payment information will be persisted under this prefix.
pub(crate) const PAYMENT_INFO_PERSISTENCE_PRIMARY_NAMESPACE: &str = "payments";
pub(crate) const PAYMENT_INFO_PERSISTENCE_SECONDARY_NAMESPACE: &str = "";

/// The spendable output information used to persisted under this prefix until LDK Node v0.3.0.
pub(crate) const DEPRECATED_SPENDABLE_OUTPUT_INFO_PERSISTENCE_PRIMARY_NAMESPACE: &str =
	"spendable_outputs";
pub(crate) const DEPRECATED_SPENDABLE_OUTPUT_INFO_PERSISTENCE_SECONDARY_NAMESPACE: &str = "";

/// RapidGossipSync's `latest_sync_timestamp` will be persisted under this key.
pub(crate) const LATEST_RGS_SYNC_TIMESTAMP_PRIMARY_NAMESPACE: &str = "";
pub(crate) const LATEST_RGS_SYNC_TIMESTAMP_SECONDARY_NAMESPACE: &str = "";
pub(crate) const LATEST_RGS_SYNC_TIMESTAMP_KEY: &str = "latest_rgs_sync_timestamp";

/// The last time we broadcast a node announcement will be persisted under this key.
pub(crate) const LATEST_NODE_ANN_BCAST_TIMESTAMP_PRIMARY_NAMESPACE: &str = "";
pub(crate) const LATEST_NODE_ANN_BCAST_TIMESTAMP_SECONDARY_NAMESPACE: &str = "";
pub(crate) const LATEST_NODE_ANN_BCAST_TIMESTAMP_KEY: &str = "latest_node_ann_bcast_timestamp";
