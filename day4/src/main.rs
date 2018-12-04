use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let events = read_input()?;

    part1(&events);

    Ok(())
}

fn read_input() -> Result<Vec<Event>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let mut events: Vec<Event> = input.lines().map(|s| parse_event(s)).collect();
    events.sort();
    Ok(events)
}

fn parse_event(s: &str) -> Event {
    lazy_static! {
        static ref time_re: Regex = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\]").unwrap();
        static ref begins_shift_re: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    }

    let time_caps = time_re.captures(s).unwrap();
    let time = Time {
        year: time_caps[1].parse().unwrap(),
        month: time_caps[2].parse().unwrap(),
        day: time_caps[3].parse().unwrap(),
        hour: time_caps[4].parse().unwrap(),
        minute: time_caps[5].parse().unwrap(),
    };
    let s = &s[(time_caps[0].len() + 1)..];
    let activity = match s {
        "wakes up" => Activity::WakesUp,
        "falls asleep" => Activity::FallsAsleep,
        _ => {
            let begins_shift_caps = begins_shift_re.captures(s).unwrap();
            Activity::BeginShift(begins_shift_caps[1].parse().unwrap())
        },
    };

    Event {
        time,
        activity,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
struct Time {
    year: u16,
    month: u16,
    day: u16,
    hour: u8,
    minute: u8,
}

impl Time {
    fn sub(&self, other: &Time) -> usize {
        // we can ignore all fields except minute
        (self.minute - other.minute) as usize
    }
}

#[derive(Debug, Eq)]
struct Event {
    time: Time,
    activity: Activity,
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time == other.time
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Activity {
    BeginShift(usize),
    FallsAsleep,
    WakesUp,
}

fn part1(events: &[Event]) {
    let mut guards: HashMap<usize,Guard> = HashMap::new();
    let mut current_guard = None;
    let mut sleep_start = Time::default();

    for event in events {
        match event.activity {
            Activity::BeginShift(id) => {
                if !guards.contains_key(&id) {
                    guards.insert(id, Guard{
                        id,
                        total_asleep: 0,
                        asleep: HashMap::new(),
                    });
                }
                current_guard = Some(id);
            },
            Activity::FallsAsleep => {
                sleep_start = event.time.clone();
            },
            Activity::WakesUp => {
                let sleep_len = event.time.sub(&sleep_start);
                if let Some(guard) = guards.get_mut(&current_guard.unwrap()) {
                    guard.total_asleep += sleep_len as usize;
                    for min in sleep_start.minute..event.time.minute {
                        *guard.asleep.entry(min).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let most_asleep_guard = guards.values().max_by_key(|g| g.total_asleep).unwrap();
    let (most_asleep_min, _) = most_asleep_guard.asleep.iter().max_by_key(|(_, count)| *count).unwrap();

    let result = most_asleep_guard.id * (*most_asleep_min as usize);

    println!("part1: {}", result);
}

#[derive(Debug)]
struct Guard {
    id: usize,
    total_asleep: usize,
    asleep: HashMap<u8,usize>,
}
