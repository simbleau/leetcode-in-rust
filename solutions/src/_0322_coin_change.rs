// Dynamic programming
// Time Complexity: O(M*N)
// Auxiliary Space: O(M*N)

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // The minimum number of coins per amount, up to the target amount
    let mut memoization = vec![-1; (amount + 1) as usize];
    memoization[0] = 0;

    for coin in coins {
        for i in 1..memoization.len() {
            if coin > i as i32 {
                // Coin doesn't go into this amount
                continue;
            } else {
                // Solution here takes the min coins of the index minus this
                // coin's value and adding 1 (which adds this coin)
                let new_min_coins = memoization[i - coin as usize] + 1;

                // All coins are initalized to -1, so it must be > 0 to be a
                // legitimate solution.
                if new_min_coins > 0 {
                    let prev_min_coins = memoization[i];
                    if prev_min_coins == -1 {
                        memoization[i] = new_min_coins;
                    } else {
                        memoization[i] =
                            std::cmp::min(prev_min_coins, new_min_coins);
                    }
                }
            }
        }
    }

    // Give the last min_coins, e.g. the target 'amount'
    *memoization.last().unwrap()
}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
}

#[test]
fn test_2() {
    assert_eq!(coin_change(vec![2], 3), -1);
}

#[test]
fn test_3() {
    assert_eq!(coin_change(vec![1], 0), 0);
}
