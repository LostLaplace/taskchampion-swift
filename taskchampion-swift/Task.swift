//
//  Task.swift
//  taskchampion-swift
//
//  Created by Cameron Johnson on 3/23/25.
//

internal import TCSwiftbridge;

class Task {
    public var uuid: String;
    public var attributes: [String: String];
    
    init(fromUuid: String) {
        uuid = fromUuid;
        attributes = [:];
    }
    
    init(fromTaskChampionTask: TCSwiftbridge.Task) {
        let fields = fromTaskChampionTask.get_fields();
        attributes = [:];
        
        for field in fields {
            attributes[field.get_key().toString()] = field.get_value().toString();
        }
        
        uuid = attributes["uuid"]!;
    }
    
    static func generate_uuid4() -> TCSwiftbridge.Uuid {
        return TCSwiftbridge.uuid_v4();
    }
    
    static func uuid4_from_string(uuid: String) throws -> TCSwiftbridge.Uuid {
        return try TCSwiftbridge.uuid_from_string(uuid);
    }
}

