#[derive(Queryable)]
pub struct VorOblSettlement {
    pub id: i64,
    pub district_title: String,
    pub settlement_id: i64,
    pub settlement_title: String,
}
