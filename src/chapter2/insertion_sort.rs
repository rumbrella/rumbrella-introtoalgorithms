//! 2.1 - Inserttion Sort
//!
//! Straight implementation of the insertion sort algorithm as described directly as a Rust function

use std::fmt::Debug;

/// Sorts in-place a mutable slice of items that are Orderable
///
/// # Examples
/// ```
/// use rumbrella_introtoalgorithms::chapter2::insertion_sort::sort_slice_mut;
///
/// let mut nums = [ 8, 3, 5, 1, 9, 7, 0, 2, 7, 4, 6, 3 ];
/// sort_slice_mut( &mut nums );
/// let expected_nums = [ 0, 1, 2, 3, 3, 4, 5, 6, 7, 7, 8, 9 ];
/// assert_eq!( expected_nums, nums, "expected {:?}, got {:?}", expected_nums, nums );
///
/// let mut nums : [i32;0] = [];
/// sort_slice_mut( &mut nums );
/// let expected_nums : [i32;0] = [];
/// assert_eq!( expected_nums, nums, "expected {:?}, got {:?}", expected_nums, nums );
/// ```
pub fn sort_slice_mut<T: Ord + Copy>(slice: &mut [T]) {
	for j in 1..slice.len() {
		let key = slice[j];
		let mut i = (j - 1) as isize;
		while i >= 0 && slice[i as usize] > key {
			slice[(i + 1) as usize] = slice[i as usize];
			i -= 1;
		}
		slice[(i + 1) as usize] = key;
	}
}


/// Sorts in-place a mutable slice of items that are Orderable
///
/// # Examples
/// ```
/// use rumbrella_introtoalgorithms::chapter2::insertion_sort::sort_slice_mut_2;
///
/// let mut nums = [ 8, 3, 5, 1, 9, 7, 0, 2, 7, 4, 6, 3 ];
/// sort_slice_mut_2( &mut nums );
/// let expected_nums = [ 0, 1, 2, 3, 3, 4, 5, 6, 7, 7, 8, 9 ];
/// assert_eq!( expected_nums, nums, "expected {:?}, got {:?}", expected_nums, nums );
///
/// let mut nums : [i32;0] = [];
/// sort_slice_mut_2( &mut nums );
/// let expected_nums : [i32;0] = [];
/// assert_eq!( expected_nums, nums, "expected {:?}, got {:?}", expected_nums, nums );
/// ```
pub fn sort_slice_mut_2<T: Ord>(slice: &mut [T]) {
	for j in 1..slice.len() {
		for i in (0..j).rev() {
			if slice[i] > slice[i + 1] {
				slice.swap(i, i + 1);
			} else {
				break;
			}
		}
	}
}


