// This project contains the source for all my leetcode solutions and practice
// problems.
//
// Authored by Spencer C. Imbleau

pub mod _0001_two_sum;
pub mod _0003_longest_palindromic_substring;
pub mod _0005_longest_substring_without_repeating_characters;
pub mod _0011_container_with_most_water;
pub mod _0015_3sum;
pub mod _0019_remove_nth_node_from_end_of_list;
pub mod _0020_valid_parentheses;
pub mod _0021_merge_two_sorted_lists;
pub mod _0033_search_in_rotated_sorted_array;
pub mod _0039_combination_sum;
pub mod _0048_rotate_image;
pub mod _0049_group_anagrams;
pub mod _0053_maximum_subarray;
pub mod _0054_spiral_matrix;
pub mod _0055_jump_game;
pub mod _0056_merge_intervals;
pub mod _0062_unique_paths;
pub mod _0070_climbing_stairs;
pub mod _0073_set_matrix_zeroes;
pub mod _0100_same_tree;
pub mod _0121_best_time_to_buy_and_sell_stock;
pub mod _0217_contains_duplicate;
pub mod _0235_lowest_common_ancestor_of_a_binary_search_tree;
pub mod _0237_delete_node_in_a_linked_list;
pub mod _0238_product_of_array_except_self;
pub mod _0242_valid_anagram;
pub mod _0253_meeting_rooms_ii;
pub mod _0322_coin_change;
pub mod _0371_sumof_two_integers;
pub mod _0647_palindromic_substrings;
pub mod _0698_paritition_to_k_equal_sum_subsets;
pub mod _1239_maximum_length_of_a_concatenated_string_with_unique_characters;
pub mod _1304_find_n_unique_integers_sum_up_to_zero;
pub mod _1342_number_of_steps_to_reduce_a_number_to_zero;
pub mod _1448_count_good_nodes_in_binary_tree;
pub mod _1647_minimum_deletions_to_make_character_frequencies_unique;

// Template for a solution
pub fn template() {}
#[cfg(test)]
#[test]
fn test_1() {
    assert_eq!(template(), ());
}

// Data structures for problems
use std::{cell::RefCell, rc::Rc};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
