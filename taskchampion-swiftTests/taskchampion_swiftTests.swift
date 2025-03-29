//
//  taskchampion_swiftTests.swift
//  taskchampion-swiftTests
//
//  Created by Cameron Johnson on 3/23/25.
//

import Testing
@testable import taskchampion_swift

struct taskchampion_swiftTests {

    @Test func example() async throws {
        // Write your test here and use APIs like `#expect(...)` to check expected conditions.
    }
    
    @Test func generate_uuid() {
        let uuid = taskchampion_swift.TaskWrapper.generate_uuid4();
        let uuid_str = uuid.to_string().toString();
        let uuid2 = try! taskchampion_swift.TaskWrapper.uuid4_from_string(uuid: uuid_str);
        #expect(uuid_str == uuid2.to_string().toString());
    }
    

}
