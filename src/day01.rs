// Copyright (c) 2025 Bastiaan Marinus van de Weerd


enum Dir { L, R }

struct Rot { dir: Dir, dist: usize }


fn part1_impl(rots: impl Iterator<Item = Rot>) -> usize {
	let mut pos = 50_usize;
	let mut count = 0_usize;
	for rot in rots {
		if rot.dist != 0 {
			match rot.dir {
				Dir::L => pos += 100 * ((rot.dist - 1) / 100 + 1) - rot.dist,
				Dir::R => pos += rot.dist,
			}
			pos %= 100;
		}
		if pos == 0 {
			count += 1;
		}
	}
	count
}

pub(crate) fn part1() -> usize {
	part1_impl(parsing::try_rots(include_str!("day01.txt")).map(Result::unwrap))
}


fn part2_impl(rots: impl Iterator<Item = Rot>) -> usize {
	let mut pos = 50_usize;
	let mut count = 0_usize;
	for rot in rots {
		let full_rots = rot.dist / 100;
		count += full_rots;
		let dist = rot.dist - full_rots * 100;

		if dist == 0 { continue }

		let prev_pos = pos;

		match rot.dir {
			Dir::L => pos += 100 - dist,
			Dir::R => pos += dist,
		}
		pos %= 100;

		if prev_pos == 0 { continue }

		match rot.dir {
			_ if pos == 0 => count += 1,
			Dir::L if prev_pos < pos => count += 1,
			Dir::R if prev_pos > pos => count += 1,
			_ => {}
		}
	}
	count
}

pub(crate) fn part2() -> usize {
	part2_impl(parsing::try_rots(include_str!("day01.txt")).map(Result::unwrap))
}


mod parsing {
	use std::{num::ParseIntError, str::FromStr};
	use super::{Dir, Rot};

	#[allow(dead_code)]
	#[derive(Debug)]
	pub(super) enum DirError {
		Len(usize),
		Char(char),
	}

	impl FromStr for Dir {
		type Err = DirError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let Some(&[b]) = s.as_bytes().get(..) else {
				return Err(DirError::Len(s.len()));
			};
			match b {
				b'L' => Ok(Self::L),
				b'R' => Ok(Self::R),
				_ => Err(DirError::Char(char::from(b)))
			}
		}
	}

	#[allow(dead_code)]
	#[derive(Debug)]
	pub(super) enum RotError {
		Empty,
		Dir(DirError),
		Dist(ParseIntError),
	}

	impl FromStr for Rot {
		type Err = RotError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			if s.is_empty() {
				return Err(RotError::Empty);
			}
			let (dir, dist) = s.split_at(1);
			let dir = dir.parse().map_err(|e| RotError::Dir(e))?;
			let dist = dist.parse().map_err(RotError::Dist)?;
			Ok(Self { dir, dist })
		}
	}

	pub(super) fn try_rots(input: &str) -> impl Iterator<Item = Result<Rot, RotError>> + use<'_> {
		input.lines().map(|line| line.parse())
	}
}


#[test]
fn tests() {
	const INPUT: &str = indoc::indoc! {"
		L68
		L30
		R48
		L5
		R60
		L55
		L1
		L99
		R14
		L82
	"};
	assert_eq!(part1_impl(INPUT.lines().map(|line| line.parse().unwrap())), 3);
	assert_eq!(part1(), 1043);
	assert_eq!(part2_impl(INPUT.lines().map(|line| line.parse().unwrap())), 6);
	assert_eq!(part2(), 5963);
}
