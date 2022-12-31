fn char_indicates_win(c: char) -> bool {
    c == 'Z'
}

fn char_indicates_draw(c: char) -> bool {
    c == 'Y'
}

fn get_winning_shape_against(c: char) -> char {
    match c {
        'A' => 'B',
        'B' => 'C',
        _ => 'A',
    }
}

fn get_losing_shape_against(c: char) -> char {
    match c {
        'A' => 'C',
        'B' => 'A',
        _ => 'B',
    }
}

pub fn get_shape_to_play_against_opponent_shape(opponent_shape: char, indication: char) -> char {
    if char_indicates_win(indication) {
        return get_winning_shape_against(opponent_shape);
    } else if char_indicates_draw(indication) {
        return opponent_shape.clone();
    } else {
        return get_losing_shape_against(opponent_shape);
    }
}
