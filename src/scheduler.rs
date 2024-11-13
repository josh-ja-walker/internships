use std::fmt::{self, Display};

use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::overrides::apply_overrides;


type User = String;

#[derive(Debug, Deserialize)]
pub struct Schedule {
    users: Vec<User>,
    handover_start_at: DateTime<Utc>,
    handover_interval_days: i64,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shift {
    user: User,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
}

impl Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt: &str = "%H:%M [%d-%m-%Y]";
        write!(f, "{0:<50} {1} -> {2}", self.user, self.start_at.format(fmt), self.end_at.format(fmt))
    }
}

impl Shift {
    /* Getter for start_at field */
    pub fn start_at(&self) -> DateTime<Utc> {
        self.start_at
    }
    
    /* Getter for end_at field */
    pub fn end_at(&self) -> DateTime<Utc> {
        self.end_at
    }
    
    /* Setter for start_at field - checks validity at runtime */
    pub fn update_start_at(&mut self, start_at: DateTime<Utc>) {
        self.start_at = start_at;
        if !self.is_valid() {
            panic!("{self:#?} is not valid");
        }
    }
    
    /* Setter for end_at field - checks validity at runtime */
    pub fn update_end_at(&mut self, end_at: DateTime<Utc>) {
        self.end_at = end_at;
        if !self.is_valid() {
            panic!("{self:#?} is not valid");
        }
    }
    
    /* Check if Shift is valid - i.e., starts before it ends */
    pub fn is_valid(&self) -> bool {
        self.start_at < self.end_at
    }
}

/* Schedule all shifts */
pub fn schedule_shifts(sched: Schedule, overrides: Vec<Shift>, from: DateTime<Utc>, until: DateTime<Utc>) -> Vec<Shift> {
    let mut shifts = generate_shifts(&sched, until);
    apply_overrides(&mut shifts, overrides);
    filter_shifts(shifts, from, until)
}


/* Generate normal (without overrides) shift schedule according to schedule data */
fn generate_shifts(sched: &Schedule, until: DateTime<Utc>) -> Vec<Shift> {
    let mut shifts: Vec<Shift> = vec![]; /* List of shifts to return */
    let sched_length: i64 = (until - sched.handover_start_at).num_days(); /* Length of schedule according to start_at and until */

    /* Loop from 0 to number of shifts */
    for shift_num in 0..(sched_length / sched.handover_interval_days) {
        /* Start of shift is handover + shift number * shift length */
        let start_at: DateTime<Utc> = sched.handover_start_at + TimeDelta::days(shift_num * sched.handover_interval_days);
        
        /* Index into users using shift_number % number of users to handle overflow */
        let user: User = sched.users[shift_num as usize % sched.users.len()].clone(); 
        
        /* Add new shift */
        shifts.push(Shift {
            user, start_at, end_at: start_at + TimeDelta::days(sched.handover_interval_days)
        });
    }

    shifts /* Return shifts */
}


/* Filter out shifts not include in from - until timeframe */
fn filter_shifts(shifts: Vec<Shift>, from: DateTime<Utc>, until: DateTime<Utc>) -> Vec<Shift> {
    shifts.into_iter()
        .filter(|shift: &Shift| shift.is_valid()) /* Filter out invalid shifts */
        .filter(|shift: &Shift| from <= shift.start_at && shift.end_at <= until) /* Filter out shifts not within from - until */
        .collect::<Vec<Shift>>()
}


#[allow(dead_code)]
/* Find the shift covering a given timestamp */
pub fn find_shift(time: DateTime<Utc>, shifts: &[Shift]) -> Option<&Shift> {
    find_shift_index(time, shifts).map(|i| &shifts[i])
}

/* Perform a binary search to find index of the shift covering a given timestamp */
pub fn find_shift_index(time: DateTime<Utc>, shifts: &[Shift]) -> Option<usize> {
    let mid = shifts.len() / 2; 
    let mid_shift = shifts.get(mid)?;

    /* If middle shift contains time, return mid index */
    if (mid_shift.start_at()..=mid_shift.end_at()).contains(&time) {
        return Some(mid);
    }

    /* Split shift in two - left contains [start, mid), right contains [mid, end) */
    let (left, right) = shifts.split_at(mid); 

    return if time < mid_shift.start_at() {
        /* If time is before mid_shift's start, recurse on left half */
        find_shift_index(time, left)
    } else {
        /* Otherwise recurse on right half, sliced to exclude mid_shift */
        find_shift_index(time, &right[1..])
    };
}

