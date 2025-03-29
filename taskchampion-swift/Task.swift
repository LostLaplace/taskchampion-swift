//
//  Task.swift
//  taskchampion-swift
//
//  Created by Cameron Johnson on 3/23/25.
//

class TaskWrapper {
    public var uuid: String;
    public var attributes: [String: String];
    
    init(fromUuid: String) {
        uuid = fromUuid;
        attributes = [:];
    }
    
    init(fromTaskChampionTask: Task) {
        let fields = fromTaskChampionTask.get_fields();
        attributes = [:];
        
        for field in fields {
            attributes[field.get_key().toString()] = field.get_value().toString();
        }
        
        uuid = attributes["uuid"]!;
    }
    
    static func generate_uuid4() -> Uuid {
        return uuid_v4();
    }
    
    static func uuid4_from_string(uuid: String) throws -> Uuid {
        return try uuid_from_string(uuid);
    }
}

