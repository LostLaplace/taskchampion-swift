//! Taskchampion support in swift projects.
//!
//! Provides access to the taskchampion library and APIs for swift projects.

use taskchampion as tc;

mod task;

#[swift_bridge::bridge]
mod ffi {

    extern "Rust" {

        #[swift_bridge(already_declared)]
        type Task;

        type Replica;
        fn new_replica_in_memory() -> Replica;
        fn commit_operations(self: &mut Replica, ops: Vec<Operation>);
    }

    extern "Rust" {
        type Operation;

        fn new_operations() -> Vec<Operation>;
    }

    extern "Rust" {
        type Uuid;

        fn uuid_v4() -> Uuid;
    }
}

pub struct Replica(tc::Replica);

fn new_replica_in_memory() -> Replica {
    let replica = tc::Replica::new(tc::StorageConfig::InMemory.into_storage().unwrap());
    Replica(replica)
}

/// Utility function for Replica methods using Operations.
fn to_tc_operations(ops: Vec<Operation>) -> Vec<tc::Operation> {
    // SAFETY: Operation is a transparent newtype for tc::Operation, so a Vec of one is
    // a Vec of the other.
    unsafe { std::mem::transmute::<Vec<Operation>, Vec<tc::Operation>>(ops) }
}

impl Replica {
    fn commit_operations(&mut self, ops: Vec<Operation>) {
        self.0.commit_operations(to_tc_operations(ops));
    }
}

pub struct Operation(tc::Operation);

fn new_operations() -> Vec<Operation> {
    Vec::new()
}

fn operations_ref(ops: Vec<Operation>) -> Vec<tc::Operation> {
    // SAFETY: Operation is a transparent newtype for tc::Operation, so a Vec of one is a
    // Vec of the other.
    unsafe { std::mem::transmute::<Vec<Operation>, Vec<tc::Operation>>(ops) }
}

struct Uuid {
    v: [u8; 16],
}

impl From<Uuid> for tc::Uuid {
    fn from(value: Uuid) -> Self {
        tc::Uuid::from_bytes(value.v)
    }
}

impl From<&Uuid> for tc::Uuid {
    fn from(value: &Uuid) -> Self {
        tc::Uuid::from_bytes(value.v)
    }
}

impl From<tc::Uuid> for Uuid {
    fn from(uuid: tc::Uuid) -> Uuid {
        Uuid {
            v: *uuid.as_bytes(),
        }
    }
}

impl From<&tc::Uuid> for Uuid {
    fn from(uuid: &tc::Uuid) -> Uuid {
        Uuid {
            v: *uuid.as_bytes(),
        }
    }
}

fn uuid_v4() -> Uuid {
    tc::Uuid::new_v4().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_replica_in_memory() {
        let replica = new_replica_in_memory();
    }

    #[test]
    fn create_uuid() {
        let uuid = uuid_v4();
        assert!(uuid.v[0] != 0);
    }
}
