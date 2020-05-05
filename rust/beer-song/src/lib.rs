pub fn verse(n: u32) -> String {
    let mut song = String::new();
    if n > 0 {
        for i in n..0 {
            song.push_str(&sing(i, i - 1));
        }
    } else {
        song.push_str(&sing(n, n));
    }
    song
}

pub fn sing(start: u32, end: u32) -> String {
    let song_line = String::new();
    if start > 2 && end > 1 {
        let song_line = (
            "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.
",
            start,
            start,
            end,
        );
    } else if start == 2 && end == 1 {
        let song_line = (
            "{} bottles of beer on the wall, {} bottles of beer.
Take one down and pass it around, {} bottle of beer on the wall.
",
            start,
            start,
            end,
        );
    } else if start == 1 && end == 0 {
        let song_line = (
            "{} bottle of beer on the wall, {} bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
",
            start,
            start,
        );
    } else if start == 0 && end == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
"
        .to_string();
    }
    song_line
}
