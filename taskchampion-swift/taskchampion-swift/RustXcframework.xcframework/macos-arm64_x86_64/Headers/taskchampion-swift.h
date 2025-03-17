// File automatically generated by swift-bridge.
#include <stdint.h>
#include <stdbool.h>
typedef struct Replica Replica;
void __swift_bridge__$Replica$_free(void* self);

void* __swift_bridge__$Vec_Replica$new(void);
void __swift_bridge__$Vec_Replica$drop(void* vec_ptr);
void __swift_bridge__$Vec_Replica$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Replica$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Replica$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Replica$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Replica$len(void* vec_ptr);
void* __swift_bridge__$Vec_Replica$as_ptr(void* vec_ptr);

typedef struct Operation Operation;
void __swift_bridge__$Operation$_free(void* self);

void* __swift_bridge__$Vec_Operation$new(void);
void __swift_bridge__$Vec_Operation$drop(void* vec_ptr);
void __swift_bridge__$Vec_Operation$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Operation$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Operation$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Operation$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Operation$len(void* vec_ptr);
void* __swift_bridge__$Vec_Operation$as_ptr(void* vec_ptr);

typedef struct TaskData TaskData;
void __swift_bridge__$TaskData$_free(void* self);

void* __swift_bridge__$Vec_TaskData$new(void);
void __swift_bridge__$Vec_TaskData$drop(void* vec_ptr);
void __swift_bridge__$Vec_TaskData$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_TaskData$pop(void* vec_ptr);
void* __swift_bridge__$Vec_TaskData$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_TaskData$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_TaskData$len(void* vec_ptr);
void* __swift_bridge__$Vec_TaskData$as_ptr(void* vec_ptr);

typedef struct Uuid Uuid;
void __swift_bridge__$Uuid$_free(void* self);

void* __swift_bridge__$Vec_Uuid$new(void);
void __swift_bridge__$Vec_Uuid$drop(void* vec_ptr);
void __swift_bridge__$Vec_Uuid$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_Uuid$pop(void* vec_ptr);
void* __swift_bridge__$Vec_Uuid$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_Uuid$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_Uuid$len(void* vec_ptr);
void* __swift_bridge__$Vec_Uuid$as_ptr(void* vec_ptr);

void* __swift_bridge__$new_replica_in_memory(void);
void* __swift_bridge__$new_replica_on_disk(void* taskdb_dir, bool create_if_missing, bool read_write);
void* __swift_bridge__$Replica$all_task_data(void* self);
void __swift_bridge__$Replica$commit_operations(void* self, void* ops);
void* __swift_bridge__$new_operations(void);
void* __swift_bridge__$create_task(void* uuid, void* ops);
void* __swift_bridge__$TaskData$get_uuid(void* self);
bool __swift_bridge__$TaskData$has(void* self, void* property);
void* __swift_bridge__$TaskData$properties(void* self);
void __swift_bridge__$TaskData$update(void* self, void* property, void* value, void* ops);
void __swift_bridge__$TaskData$update_remove(void* self, void* property, void* ops);
void __swift_bridge__$TaskData$delete_task(void* self, void* ops);
void* __swift_bridge__$uuid_v4(void);


