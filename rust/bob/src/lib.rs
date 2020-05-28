pub fn reply(message: &str) -> &str {
    if (message.trim().ends_with('!') && message == message.to_uppercase())
        || (message.contains(char::is_alphabetic)
            && message == message.to_uppercase()
            && !message.contains('?'))
    {
        return "Whoa, chill out!";
    } else if message.trim().ends_with('?')
        && message.contains(char::is_alphabetic)
        && message == message.to_uppercase()
    {
        return "Calm down, I know what I'm doing!";
    } else if message.trim().ends_with('?') {
        return "Sure.";
    } else if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    "Whatever."
}
