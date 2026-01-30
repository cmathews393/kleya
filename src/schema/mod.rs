struct kleyaQuota {
    totalStorage: i64,
    usedStorage: i64,
    entryCount: i32,
}

struct kleyaNode {
    id: uuid::Uuid,
    name: String,
    lastCheckin: Option<chrono::NaiveDateTime>,
    firstCheckin: Option<chrono::NaiveDateTime>,
    lastReportedQuota: Option<kleyaQuota>,
    routableAddress: Option<String>,
}

struct kleyaNodeList {
    nodes: Vec<kleyaNode>,
}
