import Foundation

struct MatchBriefInfo: Identifiable {
    var id: UUID

    var event: EventInfo
    var teams: [TeamCompletedMatchBriefInfo]
    var scheduled_timestamp: Int64?
    
    /// Copies data from the given `matchBriefInfoWrapper` in order to create this struct.
    init?(_ from: matchBriefInfoWrapper) {
        assert(from.teams_len == 2)
        self.teams = [TeamCompletedMatchBriefInfo]()
        
        guard let event = EventInfo(from.event) else {
            return nil
        }
        self.event = event
        
        for i in 0..<2 {
            guard let team = TeamCompletedMatchBriefInfo(from.teams_ptr[i]) else {
                return nil
            }
            self.teams.append(team)
        }
        
        if from.scheduled_timestamp.is_present == true {
            self.scheduled_timestamp = from.scheduled_timestamp.val
        } else {
            self.scheduled_timestamp = nil
        }
        
        id = UUID()
    }
}

struct EventInfo {
    var name: String
    var series: String
    
    /// Copies data from the given `eventInfoWrapper` in order to create this struct.
    init?(_ from: eventInfoWrapper) {
        guard let name = String(validatingUTF8: from.name) else {
            return nil
        }
        guard let series = String(validatingUTF8: from.series) else {
            return nil
        }
        
        self.name = name
        self.series = series
    }
}

struct TeamCompletedMatchBriefInfo {
    var name: String
    var maps_won: UInt8?
    
    /// Copies data from the given `teamCompletedMatchBriefInfoWrapper` in order to create this struct.
    init?(_ from: teamCompletedMatchBriefInfoWrapper) {
        guard let name = String(validatingUTF8: from.name) else {
            return nil
        }
        self.name = name
        
        if from.maps_won.is_present == true {
            self.maps_won = from.maps_won.val
        } else {
            self.maps_won = nil
        }
    }
}
