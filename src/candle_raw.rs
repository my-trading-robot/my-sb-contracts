use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "candles-raw")]
#[derive(Serialize, Deserialize)]
pub struct CandleRawMySbContract {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(double, tag = "3")]
    pub o: f64,
    #[prost(double, tag = "4")]
    pub c: f64,
    #[prost(double, tag = "5")]
    pub h: f64,
    #[prost(double, tag = "6")]
    pub l: f64,
    #[prost(double, tag = "7")]
    pub v: f64,
    #[prost(double, tag = "8")]
    pub src: f64,
}

impl service_sdk::rust_extensions::sorted_vec::EntityWithStrKey for CandleRawMySbContract {
    fn get_key(&self) -> &str {
        self.id.as_str()
    }
}
