import Foundation

enum MatchBriefType: UInt8 {
    case InFuture, Live, Completed
}

class MainPageScraper {
    let ptr: OpaquePointer
    
    init(_ ptr: OpaquePointer) {
        self.ptr = ptr
    }
    
    deinit {
        mainPageScraperFree(self.ptr)
    }
    
    func matchesBrief(_ type: MatchBriefType) -> [MatchBriefInfo]? {
        var matches = [MatchBriefInfo]()
        var matches_ptr: UnsafeMutablePointer<matchBriefInfoWrapper>? = nil
        let matches_len = UnsafeMutablePointer<Int32>.allocate(capacity: 1)
        
        mainPageScraperGetMatchesBrief(self.ptr, type.rawValue, &matches_ptr, matches_len)
        
        for i in 0..<matches_len.pointee {
            guard let match = MatchBriefInfo(matches_ptr![Int(i)]) else {
                return nil
            }
            matches.append(match)
        }
        
        matchesBriefInfoVecFree(matches_ptr, matches_len.pointee)
        return matches
    }
}
