use std::fmt::{self, Display};

use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};

use crate::{helpers, overrides};


type User = String;

#[derive(Debug, Deserialize)]
pub struct Schedule {
    users: Vec<User>,
    handover_start_at: DateTime<Utc>,
    handover_interval_days: i64,
}

impl Schedule {
    pub fn handover_start_at(&self) -> DateTime<Utc> {
        self.handover_start_at
    }
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
    pub fn set_start_at(&mut self, start_at: DateTime<Utc>) {
        self.start_at = start_at;
        self.print_warning();
    }
    
    /* Setter for end_at field - checks validity at runtime */
    pub fn set_end_at(&mut self, end_at: DateTime<Utc>) {
        self.end_at = end_at;
        self.print_warning();
    }
    
    /* Check if Shift is valid - i.e., starts before it ends */
    pub fn is_valid(&self) -> bool {
        self.start_at < self.end_at
    }
    
    /* Check if Shift is valid - i.e., starts before it ends */
    fn print_warning(&self) {
        if !self.is_valid() {
            println!("Warning! {self} is an invalid shift");
        }
    }
}


/* Schedule all shifts */
pub fn schedule_shifts(sched: Schedule, overrides: Vec<Shift>, from: DateTime<Utc>, until: DateTime<Utc>) -> Vec<Shift> {
    let mut shifts = generate_shifts(&sched, until);
    overrides::apply_overrides(&mut shifts, overrides);
    helpers::truncate_shifts(shifts, &sched, from, until)
}


/* Generate normal (without overrides) shift schedule according to schedule data */
fn generate_shifts(schedule: &Schedule, until: DateTime<Utc>) -> Vec<Shift> {
    let mut shifts: Vec<Shift> = vec![]; /* List of shifts to return */
    let sched_length: i64 = (until - schedule.handover_start_at).num_days(); /* Length of schedule according to start_at and until */

    /* Loop from 0 to number of shifts */
    for shift_num in 0..(sched_length / schedule.handover_interval_days) {
        /* Start of shift is handover + shift number * shift length */
        let start_at: DateTime<Utc> = schedule.handover_start_at + TimeDelta::days(shift_num * schedule.handover_interval_days);
        
        /* Index into users using shift_number % number of users to handle overflow */
        let user: User = schedule.users[shift_num as usize % schedule.users.len()].clone(); 
        
        /* Add new shift */
        shifts.push(Shift {
            user, start_at, end_at: start_at + TimeDelta::days(schedule.handover_interval_days)
        });
    }

    shifts /* Return shifts */
}

