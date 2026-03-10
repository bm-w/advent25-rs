// Copyright (c) 2025 Bastiaan Marinus van de Weerd


#[cfg_attr(test, derive(Debug))]
struct Range(std::ops::Range<usize>);


#[allow(dead_code)]
fn part1_naive(ranges: impl Iterator<Item = Range>) -> usize {
	let mut sum = 0usize;

	for range in ranges {
		let digits_start = digits10(range.0.start);
		let digits_end = digits10(range.0.end);
		debug_assert!(digits_start.abs_diff(digits_end) <= 1);
		let digits = if digits_start % 2 == 0 { digits_start }
			else if digits_end % 2 == 0 { digits_end }
			else { continue };

		let div = 10usize.pow((digits >> 1) as _);
		let from = if digits == digits_start { range.0.start / div } else { div / 10 };
		let to = if digits == digits_end { range.0.end / div } else { div - 1 };

		for h in from..=to {
			let x = h * div + h;
			if x < range.0.start { continue }
			if x > range.0.end { break }

			sum += x;
		}
	}

	sum
}

fn part1_impl(ranges: impl Iterator<Item = Range>) -> usize {
	let mut sum = 0usize;

	for range in ranges {
		let digits_start = digits10(range.0.start);
		let digits_end = digits10(range.0.end);
		debug_assert!(digits_start.abs_diff(digits_end) <= 1);
		let digits = if digits_start % 2 == 0 { digits_start }
			else if digits_end % 2 == 0 { digits_end }
			else { continue };

		let base = 10usize.pow((digits >> 1) as _) + 1;

		let start = range.0.start.max(10usize.pow((digits - 1) as _));
		let end = range.0.end.min(10usize.pow(digits as _) - 1);

		let from = ((start - 1) / base + 1) * base;
		let to = (end / base) * base;
		if to < from { continue }
		let count = (to - from) / base + 1;
		sum += count * (from + to) / 2;
	}

	sum
}

pub(crate) fn part1() -> usize {
	part1_impl(parsing::try_ranges(include_str!("day02.txt")).map(Result::unwrap))
}


pub(crate) fn part2() -> &'static str {
	"WIP"
}


mod parsing {
	use std::{num::ParseIntError, str::FromStr};
	use super::Range;

	#[allow(dead_code)]
	#[derive(Debug)]
	pub(super) enum RangeError {
		Format,
		Start(ParseIntError),
		End(ParseIntError),
		Rev,
	}

	impl FromStr for Range {
		type Err = RangeError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let (start, end) = s.split_once('-').ok_or(RangeError::Format)?;
			let start = start.parse().map_err(RangeError::Start)?;
			let end = end.parse().map_err(RangeError::End)?;
			if start >= end { return Err(RangeError::Rev) }
			Ok(Self(start..end))
		}
	}

	pub(super) fn try_ranges(input: &str)
	-> impl Iterator<Item = Result<Range, RangeError>> + use<'_> {
		input.lines()
			.flat_map(|line| line.split(',').filter(|part| !part.is_empty()))
			.map(|part| part.parse())
	}
}


#[test]
fn tests() {
	const INPUT: &str = indoc::indoc! {"
		11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
		1698522-1698528,446443-446449,38593856-38593862,565653-565659,
		824824821-824824827,2121212118-2121212124
	"};
	assert_eq!(part1_naive(parsing::try_ranges(INPUT).map(Result::unwrap)), 1227775554);
	assert_eq!(part1_impl(parsing::try_ranges(INPUT).map(Result::unwrap)), 1227775554);
	assert_eq!(part1(), 44487518055);
}


/// Can overflow, but inputs are small enough.
fn digits10(x: usize) -> usize {
	if x <= 1 { return 1 }
	let mut r = 1;
	let mut n = 1;
	while r <= x {
		r *= 10;
		n += 1;
	}
	n - 1
}


#[test]
fn digits10_tests() {
	assert_eq!(digits10(0), 1);
	assert_eq!(digits10(1), 1);
	assert_eq!(digits10(2), 1);
	assert_eq!(digits10(9), 1);
	assert_eq!(digits10(10), 2);
	assert_eq!(digits10(11), 2);
	assert_eq!(digits10(99), 2);
	assert_eq!(digits10(100), 3);
	assert_eq!(digits10(101), 3);
	assert_eq!(digits10(999), 3);
	assert_eq!(digits10(1000), 4);
	assert_eq!(digits10(1001), 4);
}
