#[swift_bridge::bridge]
pub mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    pub struct Task {
        fields: Vec<TaskKeyValue>,
        uuid: String
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct TaskKeyValue {
        key: String,
        value: String
    }
}
