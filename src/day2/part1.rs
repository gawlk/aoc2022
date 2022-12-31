fn is_rock(c: char) -> bool {
    c == 'A' || c == 'X'
}

fn is_paper(c: char) -> bool {
    c == 'B' || c == 'Y'
}

fn is_scissors(c: char) -> bool {
    c == 'C' || c == 'Z'
}

pub fn get_shape_value(c: char) -> i32 {
    if is_rock(c) {
        1
    } else if is_paper(c) {
        2
    } else {
        3
    }
}

pub fn get_round_score(c1: char, c2: char) -> i32 {
    if get_shape_value(c1) == get_shape_value(c2) {
        3
    } else if (is_rock(c1) && is_scissors(c2))
        || (is_paper(c1) && is_rock(c2))
        || (is_scissors(c1) && is_paper(c2))
    {
        6
    } else {
        0
    }
}
