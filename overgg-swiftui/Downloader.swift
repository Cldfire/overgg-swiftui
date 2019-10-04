import Foundation

class Downloader {
    let ptr: OpaquePointer
    
    init() {
        var dlr: OpaquePointer? = nil
        
        downloaderNew(&dlr)
        self.ptr = dlr!
    }
    
    deinit {
        downloaderFree(ptr)
    }
    
    func mainPage() -> MainPageScraper? {
        var scraper: OpaquePointer? = nil
        
        if downloaderGetMainPage(ptr, &scraper) == 0 {
            return MainPageScraper(scraper!)
        } else {
            return nil
        }
    }
}
