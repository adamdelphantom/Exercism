pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if !list.is_empty() {
        for (i, x) in list.iter().enumerate() {
            if i + 1 == list.len() {
                break;
            }
            let line = format!("For want of a {} the {} was lost.\n", x, list[i + 1]);
            proverb.push_str(&line);
        }
        let last_line = format!("And all for the want of a {}.", list[0]);
        proverb.push_str(&last_line);
    }
    proverb
}
