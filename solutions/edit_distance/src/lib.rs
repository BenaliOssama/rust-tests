pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len() + 1;
    let n = target.len() + 1;
    let mut dp = vec![vec![0; n]; m];

    // Initialize the first row and column
    for i in 0..m {
        dp[i][0] = i;
    }
    for j in 0..n {
        dp[0][j] = j;
    }

    // Fill in the rest of the matrix
    for i in 1..m {
        for j in 1..n {

            /*____________________________logic__________________________*/
            let cost = if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

            dp[i][j] = usize::min(
                dp[i - 1][j] + 1, // Deletion
                usize::min(
                    dp[i][j - 1] + 1, // Insertion
                    dp[i - 1][j - 1] + cost, // Substitution or no operation
                ),
            );
            /*______________________________________________________________*/

        }
    }

    dp[m-1][n-1]
}
