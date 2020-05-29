pub fn brackets_are_balanced(string: &str) -> bool {
    let mut openers = vec![];
    let mut closers = vec![];
    for c in string.chars() {
        if c == '(' || c == '[' || c == '{' {
            openers.push(c);
        } else if c == ')' || c == ']' || c == '}' {
            closers.push(c);
            if c == ')' && !openers.contains(&'(')
                || c == ']' && !openers.contains(&'[')
                || c == '}' && !openers.contains(&'{')
            {
                return false;
            }
        }
    }
    if openers == closers {
        return true;
    }
    false
}
