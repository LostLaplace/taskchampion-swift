use taskchampion as tc;

#[swift_bridge::bridge]
pub mod ffi {

    extern "Rust" {
        type Uuid;
        fn uuid_v4() -> Uuid;
        fn uuid_from_string(uuid: String) -> Result<Uuid, String>;
        fn to_string(self: Uuid) -> String;
    }

    extern "Rust" {
        type TaskKeyValue;
        type Task;

        fn get_fields(self: &Task) -> Vec<TaskKeyValue>;
        fn get_key(self: &TaskKeyValue) -> String;
        fn get_value(self: &TaskKeyValue) -> String;
    }
}

// UUID
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Uuid {
    v: [u8; 16],
}

impl From<tc::Uuid> for Uuid {
    fn from(uuid: tc::Uuid) -> Uuid {
        Uuid {
            v: *uuid.as_bytes()
        }
    }
}

impl From<Uuid> for tc::Uuid {
    fn from(uuid: Uuid) -> tc::Uuid {
        tc::Uuid::from_bytes(uuid.v)
    }
}

fn uuid_v4() -> Uuid {
   tc::Uuid::new_v4().into() 
}

fn uuid_from_string(uuid: String) -> Result<Uuid, String> {
    tc::Uuid::parse_str(&uuid).map(|i| i.into()).map_err(|i| i.to_string())
}

impl Uuid {
    fn to_string(self: Uuid) -> String {
        tc::Uuid::from(self).as_hyphenated().to_string()
    }
}


// Task
#[derive(Clone)]
struct TaskKeyValue {
    key: String,
    value: String
}

impl TaskKeyValue {
    fn get_key(self: &TaskKeyValue) -> String {
        self.key.clone()
    }

    fn get_value(self: &TaskKeyValue) -> String {
        self.value.clone()
    }
}

struct Task {
    fields: Vec<TaskKeyValue>
}

impl Task {
    fn get_fields(self: &Task) -> Vec<TaskKeyValue> {
        self.fields.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_uuid(uuid: Uuid) -> bool {
        let mut all_zero = true;
        uuid.v.into_iter().for_each(|val| {
            if val != 0 {
                all_zero = false;
            }
        });
        !all_zero
    }

    #[test]
    fn generate_uuid() {
        let uuid = uuid_v4();
        // Ensure the uuid is not all 0s
        assert!(validate_uuid(uuid));
    }

    #[test]
    fn generate_uuid_from_string() {
        let uuid_str: String = "01020304-0506-0708-090a-0b0c0d0e0f10".to_string();
        let uuid = uuid_from_string(uuid_str);
        assert!(validate_uuid(uuid.unwrap()));
    }

    #[test]
    fn error_on_bad_uuid_string() {
        let uuid_str: String = "1234".to_string();
        let uuid = uuid_from_string(uuid_str);
        assert!(uuid.as_ref().err().is_some());
        let error = uuid.err().unwrap();
        println!("{error}");
    }

    #[test]
    fn uuid_to_string() {
        let uuid = uuid_v4();
        assert!(uuid.to_string().len() == 36);
    }
}
