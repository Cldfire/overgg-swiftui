//

import SwiftUI

struct ContentView: View {
    let matches: [MatchBriefInfo] = Downloader().mainPage()!.matchesBrief(.Completed)!
    
    var body: some View {
        VStack {
            Text("Completed Matches")
                .font(.title)

            ForEach(matches) { match in
                Text("\(match.teams[0].name) played \(match.teams[1].name)")
            }
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
