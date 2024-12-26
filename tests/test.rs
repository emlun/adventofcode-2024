// Solutions to Advent of Code 2024
// Copyright (C) 2024  Emil Lundberg <emil@emlun.se>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use adventofcode_2024::common::day_input_filename;
use adventofcode_2024::common::get_file_lines;
use adventofcode_2024::days;

fn test_day(day: u8, correct_a: &str, correct_b: &str) -> Result<(), std::io::Error> {
    let solve = days::get_solver(day).unwrap();
    let input_lines = get_file_lines(&day_input_filename(day))?;
    let (solution_a, solution_b) = solve(&input_lines);
    assert_eq!(
        solution_a.as_str(),
        correct_a,
        "Incorrect solution for day {}a",
        day
    );
    assert_eq!(
        solution_b.as_str(),
        correct_b,
        "Incorrect solution for day {}b",
        day
    );

    Ok(())
}

macro_rules! test_day {
    ($name: ident, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            let day_name = stringify!($name);
            let day_num: u8 = day_name[3..].parse().unwrap();
            test_day(day_num, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, "2815556", "23927637");
test_day!(day02, "371", "426");
test_day!(day03, "173785482", "83158140");
test_day!(day04, "2593", "1950");
test_day!(day05, "6505", "6897");
test_day!(day06, "5551", "1939");
test_day!(day07, "3598800864292", "340362529351427");
test_day!(day08, "344", "1182");
test_day!(day09, "6154342787400", "6183632723350");
test_day!(day10, "550", "1255");
test_day!(day11, "211306", "250783680217283");
test_day!(day12, "1437300", "849332");
test_day!(day13, "29201", "104140871044942");
test_day!(day14, "225521010", "7774");
test_day!(day15, "1442192", "1448458");
test_day!(day16, "85396", "428");
test_day!(day17, "3,6,3,7,0,7,0,3,0", "136904920099226");
test_day!(day18, "232", "44,64");
test_day!(day19, "342", "891192814474630");
test_day!(day20, "1323", "983905");
test_day!(day22, "15335183969", "1696");
test_day!(day23, "1337", "aw,fk,gv,hi,hp,ip,jy,kc,lk,og,pj,re,sr");
test_day!(day24, "56620966442854", "chv,jpj,kgj,rts,vvw,z07,z12,z26");
test_day!(day25, "3155", "");
