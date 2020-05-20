pub fn reply(message: &str) -> &str {
    if message == "" {
        return "Fine. Be that way!";
    } else if message.contains("?") && message.to_uppercase() == message {
        return "Calm down, I know what I'm doing!";
    } else if message.contains("?") {
        return "Sure.";
    } else if message.contains(&message.to_uppercase()) {
        return "Whoa, chill out!";
    }
    "Whatever."
}
