pub fn get_threshold_values(mut moves: &[usize], thresholds: &[usize]) -> Vec<usize> {
    let mut counts = Vec::with_capacity(thresholds.len() + 1);
    for threshold in thresholds {
        let move_number = moves.iter().take_while(|&x| x < threshold).count();
        counts.push(move_number);
        moves = &moves[move_number..];
    }
    counts.push(moves.len());
    counts
}
