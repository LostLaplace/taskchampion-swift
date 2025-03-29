# taskchampion-swift

This project aims to produce an XCFramework bundle bridging the gap between swift and the rust
taskchampion library. This project could be consumed by downstream apps like [taskchamp](https://github.com/marriagav/taskchamp)
to provide a more rich user experience and more native taskchampion features.

## Planned Features

- [ ] Task CRUD operations
- [ ] Task syncing with all taskchampion supported backends

## Todo

- [ ] Figure out how to link to the right static library for each supported target
- [ ] Remove Result<*, String> types as these are not supported by swiftbridge in a memory safe way
- [ ] Figure out how to inject 'import tc' into the generated swift code automatically


