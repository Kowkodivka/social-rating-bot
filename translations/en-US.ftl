ping = ping
    .description = Replies with pong!

ping-message = Pong! 🏓

reputation-added = Reputation successfully added!
reputation-already-given = You have already given reputation for this message!
reputation-decreased = Reputation successfully decreased!
reputation-not-found = No reputation found for this message!

message-reputation = The message has a reputation of { $reputation }.
user-reputation = User { $user_name } has a total reputation of { $reputation }.

experience = experience
    .description = Manage user experience.

experience-view = view
    .description = View your current experience.

experience-leaderboard = leaderboard
    .description = View the experience leaderboard.

experience-main-message = Use one of the subcommands: `view` to see your experience or `leaderboard` to see the leaderboard.
experience-message = Your current experience is: { $experience }

leaderboard-empty = No users found in the leaderboard.
leaderboard-entry = #{ $position }: User { $user_id } with { $experience } experience.
leaderboard-message = Here is the experience leaderboard:\n\n{ $leaderboard }

not-enough-experience = You need at least { $required_experience } experience to repute others!
