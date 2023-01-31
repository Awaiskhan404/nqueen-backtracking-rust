use std::iter;

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut results = vec![];
    let mut queens = vec![-1; n as usize];

    solve(0, n, &mut queens, &mut results);

    return results;
}

fn solve(row: i32, n: i32, queens: &mut [i32], results: &mut Vec<Vec<String>>) {
    if row == n {
        let mut board = vec![];
        for i in 0..n as usize {
            let s: String = iter::repeat('.').take(n as usize).collect();
            let mut s = s.chars().collect::<Vec<_>>();
            s[queens[i] as usize] = 'Q';
            board.push(s.iter().collect::<String>());
        }
        results.push(board);
        return;
    }

    for i in 0..n as usize {
        if is_valid(row, i as i32, queens) {
            queens[row as usize] = i as i32;
            solve(row + 1, n, queens, results);
            queens[row as usize] = -1;
        }
    }
}

fn is_valid(row: i32, col: i32, queens: &[i32]) -> bool {
    for i in 0..row as usize {
        if queens[i] == col || (row - i as i32).abs() == (col - queens[i]).abs() {
            return false;
        }
    }
    return true;
}

fn main() {
    let solutions = solve_n_queens(8);
    for solution in solutions {
        for row in solution {
            println!("{}", row);
        }
        println!("");
    }
}
