// Time Complexity: O(M*N)
// Auxiliary Space: O(M*N)

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut min_coins = vec![-1; (amount + 1) as usize];
    *min_coins.get_mut(0).unwrap() = 0;

    for coin in coins {
        for i in 1..min_coins.len() {
            if coin > i as i32 {
                // Coin doesn't go into this amount
                continue;
            } else {
                // Dynamic programming - Reduce the problem size
                // Solution here takes the min coins of the index minus this
                // coin's value and adding 1 (which adds this coin)
                let new_min_coins =
                    min_coins.get(i - coin as usize).unwrap() + 1;

                // This could give -1 + 1, so we need to filter bad solutions!
                if new_min_coins > 0 {
                    let prev_min_coins = min_coins.get_mut(i).unwrap();
                    if prev_min_coins == &-1 {
                        *prev_min_coins = new_min_coins;
                    } else {
                        *prev_min_coins =
                            std::cmp::min(*prev_min_coins, new_min_coins);
                    }
                }
            }
        }
    }
    // Give the last min_coins, e.g. the target 'amount'
    *min_coins.last().unwrap()
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
