extern crate chrono;

use std::fs::File;
use std::io::prelude::*;
use chrono::{NaiveDateTime, Timelike};
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Eq, Debug)]
struct Event {
    date: NaiveDateTime,
    action: String,
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.date == other.date
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.date.cmp(&other.date)
    }
}

fn part1(input: &String) -> u32 {
    let mut ordered_input: Vec<Event> = Vec::new();

    for line in input.lines() {
        let mut ls = line.split("] ");
        let date: String = ls.next().unwrap().chars().skip(1).collect();
        ordered_input.push(Event {
            action: ls.next().unwrap().to_string(),
            date: NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M").unwrap()
        });
    }

    // hashmap, key guard id -> vec (min, times found asleep)
    ordered_input.sort();

    let mut active_guard: u32 = 0;
    let mut active_guard_asleep = 0;
    let mut sleep_map: HashMap<u32, HashMap<u32,u32>> = HashMap::new();
    for event in ordered_input {
        match event.action.as_ref() {
            "wakes up" => {
                for min in active_guard_asleep..event.date.time().minute() {
                    let mut internal_map = sleep_map.entry(active_guard).or_default();
                    *internal_map.entry(min).or_default() +=1;
                }
            },
            "falls asleep" => active_guard_asleep = event.date.time().minute(),
            _ => active_guard = event.action.split(' ').nth(1).unwrap().chars().skip(1).collect::<String>().parse().unwrap()
        }
    }

    let mut max_slept = 0;
    let mut max_slept_guard_id = 0;
    for (guard_id, minutes) in sleep_map.iter() {
        let minutes_slept = minutes.values().fold(0, |total_slept, min| total_slept + min);
        if minutes_slept > max_slept {
            max_slept = minutes_slept;
            max_slept_guard_id = guard_id.clone();
        }
    }

    let mut mx_min = 0;
    let mut times_slept = 0;
    for (min, x_times) in sleep_map.get(&max_slept_guard_id).unwrap().iter() {
        if x_times > &times_slept {
            times_slept = *x_times;
            mx_min = *min;
        }
    }

    mx_min * max_slept_guard_id
}

fn part2(input: &String) -> u32 {
    let mut ordered_input: Vec<Event> = Vec::new();

    for line in input.lines() {
        let mut ls = line.split("] ");
        let date: String = ls.next().unwrap().chars().skip(1).collect();
        ordered_input.push(Event {
            action: ls.next().unwrap().to_string(),
            date: NaiveDateTime::parse_from_str(date.as_str(), "%Y-%m-%d %H:%M").unwrap()
        });
    }

    // hashmap, key guard id -> vec (min, times found asleep)
    ordered_input.sort();

    let mut active_guard: u32 = 0;
    let mut active_guard_asleep = 0;
    let mut sleep_map: HashMap<u32, HashMap<u32,u32>> = HashMap::new();
    for event in ordered_input {
        match event.action.as_ref() {
            "wakes up" => {
                for min in active_guard_asleep..event.date.time().minute() {
                    let mut internal_map = sleep_map.entry(active_guard).or_default();
                    *internal_map.entry(min).or_default() +=1;
                }
            },
            "falls asleep" => active_guard_asleep = event.date.time().minute(),
            _ => active_guard = event.action.split(' ').nth(1).unwrap().chars().skip(1).collect::<String>().parse().unwrap()
        }
    }
    // Iterate over guards and minutes, fold over minutes and times and max by number of times
    let ((sleepiest_min, _), guard) = sleep_map.iter().map(|(&guard, times)| {
        (times.iter().fold((0, 0), |max_pair, (&min, &times)| {
            if max_pair.1 < times {
                (min, times)
            } else {
                max_pair
            }
        }), guard)
    }).max_by_key(|((_, n), _)| *n).unwrap();

    guard * sleepiest_min
}

#[test]
fn validate_part1() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up".to_string();

    assert_eq!(240, part1(&input));
}

 #[test]
 fn validate_part2() {
     let input = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up".to_string();

    assert_eq!(4455, part2(&input));
}


