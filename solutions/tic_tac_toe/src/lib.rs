pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return String::from("player X won");
    }
    else if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return String::from("player O won");
    }
    String::from("tie")
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
     compare(player, table[0][0], table[1][1] , table[2][2]) || compare(player, table[0][2], table[1][1], table[2][0])
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if compare(player, table[i][0], table[i][1], table[i][2]) {
            return true ; 
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3 {
        if compare(player, table[0][i], table[1][i], table[2][i]) {
            return true ; 
        }
    }
    false
}

fn compare(player : char , x: char,  y: char,  z: char) -> bool{
    if player != 'X' && player != 'O' {
        return false;
    }
    if x == y && y == z  && z == player {
        return true;
    }
    false
}
