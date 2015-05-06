use std::collections::HashMap;
use error::Error;

/// Returned by `Client` API calls. A result containing an etcd `Response` or an `Error`.
pub type EtcdResult = Result<Response, Error>;

/// A successful response from etcd.
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Response {
    /// The action that was taken, e.g. `get`, `set`.
    pub action: String,
    /// The etcd `Node` that was operated upon.
    pub node: Node,
    /// The previous state of the target node.
    pub prevNode: Option<Node>,
}

/// An etcd key or directory.
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct Node {
    /// The new value of the etcd creation index.
    pub createdIndex: Option<u64>,
    /// Whether or not the node is a directory.
    pub dir: Option<bool>,
    /// An ISO 8601 timestamp for when the key will expire.
    pub expiration: Option<String>,
    /// The name of the key.
    pub key: Option<String>,
    /// The new value of the etcd modification index.
    pub modifiedIndex: Option<u64>,
    /// Child nodes of a directory.
    pub nodes: Option<Vec<Node>>,
    /// The key's time to live in seconds.
    pub ttl: Option<i64>,
    /// The value of the key.
    pub value: Option<String>,
}

/// Statistics about an etcd cluster leader.
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct LeaderStats {
    /// A unique identifier of a leader member.
    pub leader: String,
    /// Statistics for each peer in the cluster keyed by each peer's unique identifier.
    pub followers: HashMap<String, FollowerStats>,
}

/// Statistics on the health of a single follower.
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct FollowerStats {
    /// Counts of Raft RPC request successes and failures to this follower.
    pub counts: Option<CountStats>,
    /// Latency statistics for this follower.
    pub latency: Option<LatencyStats>,
}

#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct CountStats {
    pub fail: Option<u64>,
    pub success: Option<u64>,
}

#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct LatencyStats {
    pub average: Option<f64>,
    pub current: Option<f64>,
    pub maximum: Option<f64>,
    pub minimum: Option<f64>,
    pub standardDeviation: Option<f64>,
}

/// Statistics about an etcd cluster node.
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct SelfStats {
    /// A unique identifier for this member.
    pub id: String,
    /// Information about the cluster leader.
    pub leaderInfo: LeaderInfo,
    /// This node's name.
    pub name: String,
    /// Number of append requests this node has processed.
    pub recvAppendRequestCnt: u64,
    /// Number of requests that this node has sent.
    pub sendAppendRequestCnt: u64,
    /// Number of bytes per second this node is receiving (follower only).
    pub recvBandwidthRate: Option<f64>,
    /// Number of requests per second this node is receiving (follower only).
    pub recvPkgRate: Option<f64>,
    /// Number of bytes per second this node is sending (leader only). This value is undefined on single member clusters.
    pub sendBandwidthRate: Option<f64>,
    /// Number of requests per second this node is sending (leader only). This value is undefined on single member clusters.
    pub sendPkgRate: Option<f64>,
    /// Either leader or follower.
    pub state: NodeState,
    /// The time when this node was started.
    pub startTime: String,
}

/// Information about a leader, included with node statistics.
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct LeaderInfo {
    /// The leader's unique identifier.
    pub leader: String,
    /// The time the leader was started.
    pub startTime: String,
    /// The amount of time the leader has been leader.
    pub uptime: String,
}

/// State of a node
#[derive(Clone, Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub enum NodeState {
    StateLeader,
    StateFollower,
}
