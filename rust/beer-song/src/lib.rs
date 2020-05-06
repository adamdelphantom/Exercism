pub fn verse(n: u32) -> String {
    let mut line = String::new();
    if n > 2 {
        let new_line = format!(
            "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.
",
            n,
            n,
            n - 1
        );
        line.push_str(&new_line);
    } else if n == 2 {
        line.push_str(
            "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.
",
        );
    } else if n == 1 {
        line.push_str(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
",
        );
    } else {
        line.push_str(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
",
        );
    }
    line
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for i in (end..=start).rev() {
        song.push_str(&verse(i));
        if i > end {
            song.push_str("\n");
        }
    }
    song
}
