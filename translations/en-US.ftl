# Общие команды
ping = ping
    .description = Replies with pong!

ping-message = Pong! 🏓

# Репутация
repute = repute
    .description = Give reputation to a message
    .msg = msg
    .msg-description = The message to give reputation to

diminish = diminish
    .description = Remove reputation from a message
    .msg = msg
    .msg-description = The message to remove reputation from

show_message_reputation = show_message_reputation
    .description = Display the reputation of a message
    .msg = msg
    .msg-description = The message to show the reputation of

show_user_reputation = show_user_reputation
    .description = Display the total reputation of a user
    .user = user
    .user-description = The user whose reputation to display

reputation-added = Reputation successfully added!
reputation-already-given = You have already given reputation for this message!
reputation-removed = Reputation successfully removed!
reputation-not-found = No reputation found for this message!

message-reputation = The message has a reputation of { $reputation }.
user-reputation = User { $user_name } has a total reputation of { $reputation }.

# Опыт пользователя
experience = experience
    .description = Displays your current experience.

experience-message = Your current experience is: { $experience }
not-enough-experience = You need at least { $required_experience } experience to repute others!

# Ошибки
error = error
    .generic = An unexpected error occurred. Please try again later.
    .not-found = The requested item could not be found.
