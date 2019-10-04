//

import SwiftUI

struct ContentView: View {
    // TODO: set up a bindable object to store all of this and bind to it here
    let completedMatches: [MatchBriefInfo] = Downloader().mainPage()!.matchesBrief(.Completed)!
    let upcomingMatches: [MatchBriefInfo] = Downloader().mainPage()!.matchesBrief(.InFuture)!
    let liveMatches: [MatchBriefInfo] = Downloader().mainPage()!.matchesBrief(.Live)!

    var body: some View {
        NavigationView {
            List {
                Section(header: Text("Live")) {
                    ForEach(liveMatches) { match in
                        Match(match: match, isComplete: false)
                    }
                }
                
                Section(header: Text("Upcoming")) {
                    ForEach(upcomingMatches) { match in
                        UpcomingMatch(match: match)
                    }
                }
                
                Section(header: Text("Completed")) {
                    ForEach(completedMatches) { match in
                        Match(match: match, isComplete: true)
                    }
                }
            }.navigationBarTitle("over.gg app")
        }
    }
}

struct Match: View {
    var match: MatchBriefInfo
    var isComplete: Bool

    var body: some View {
        HStack {
            // TODO: stick slim green rectangle in front of winner
            VStack(alignment: .leading) {
                Text("\(match.teams[0].name)")
                    .if(isComplete && didTeamZeroWin()) { content in
                        content.bold()
                    }
                Text("\(match.teams[1].name)")
                    .if(isComplete && !didTeamZeroWin()) { content in
                        content.bold()
                    }
            }
            
            Spacer()
            
            VStack {
                Text("\(match.teams[0].maps_won!)")
                    .if(isComplete && didTeamZeroWin()) { content in
                        content.bold()
                }
                Text("\(match.teams[1].maps_won!)")
                    .if(isComplete && !didTeamZeroWin()) { content in
                        content.bold()
                    }
            }
        }
    }
    
    func didTeamZeroWin() -> Bool {
        return match.teams[0].maps_won! > match.teams[1].maps_won!
    }
}

struct UpcomingMatch: View {
    var match: MatchBriefInfo

    var body: some View {
        HStack {
            VStack(alignment: .leading) {
                Text("\(match.teams[0].name)")
                Text("\(match.teams[1].name)")
            }
            
            Spacer()
            
            VStack {
                Text(formatDate(date: Double(match.scheduled_timestamp!)))
                    .fontWeight(.light)
                    .font(.system(size: 11))
                    .italic()
            }
        }
    }

    func formatDate(date: TimeInterval) -> String {
        let dformat = DateFormatter()
        dformat.dateFormat = "MMM d, h:mm a"
        
        return dformat.string(from: Date(timeIntervalSince1970: date))
    }
}

extension View {
   func `if`<Content: View>(_ conditional: Bool, content: (Self) -> Content) -> some View {
        if conditional {
            return AnyView(content(self))
        } else {
            return AnyView(self)
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
