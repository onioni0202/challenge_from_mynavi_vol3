fn longest_common_subsequence(word1: &Vec<char>, word2: &Vec<char>) -> Vec<Vec<i32>> {
    let word1_size: usize = word1.len();
    let word2_size: usize = word2.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; word2_size + 1]; word1_size + 1];
    for idx1 in (0..word1_size).rev() {
        for idx2 in (0..word2_size).rev() {
            if word1[idx1] == word2[idx2] {
                dp[idx1][idx2] = dp[idx1 + 1][idx2 + 1] + 1;
            } else {
                dp[idx1][idx2] = i32::max(dp[idx1 + 1][idx2], dp[idx1][idx2 + 1]);
            }
        }
    }
    return dp;
}

fn string_recovery(word1: &Vec<char>, word2: &Vec<char>, dp: &Vec<Vec<i32>>) -> String {
    let word1_size: usize = word1.len();
    let word2_size: usize = word2.len();
    let mut ans: Vec<char> = vec![];
    let (mut cur_idx1, mut cur_idx2) = (0, 0);
    while cur_idx1 < word1_size && cur_idx2 < word2_size {
        if word1[cur_idx1] == word2[cur_idx2] {
            ans.push(word1[cur_idx1]);
            cur_idx1 += 1;
            cur_idx2 += 1;
        } else if dp[cur_idx1][cur_idx2] == dp[cur_idx1 + 1][cur_idx2] {
            cur_idx1 += 1;
        } else {
            cur_idx2 += 1;
        }
    }
    return ans.iter().collect();
}

fn main() {
    let word1: Vec<char> = "なんじ、ぶんがくとかがくのちからをしんじよ".chars().collect();
    let word2: Vec<char> = "じょうほうぶんせきしてかいぜんあくしょん".chars().collect();

    let dp = longest_common_subsequence(&word1, &word2);
    let ans = string_recovery(&word1, &word2, &dp);

    println!("{}", ans);
}
