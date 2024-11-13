use chrono::{DateTime, TimeDelta, Utc};
use serde::{Deserialize, Serialize};


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

/* Schedule all shifts */
pub fn schedule_shifts(sched: Schedule, _overrides: Vec<Shift>, from: DateTime<Utc>, until: DateTime<Utc>) -> Vec<Shift> {
    let shifts = generate_shifts(&sched, until);
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
        .filter(|shift: &Shift| shift.start_at < shift.end_at) /* Filter out invalid shifts */
        .filter(|shift: &Shift| from <= shift.start_at && shift.end_at <= until) /* Filter out shifts not within timeframe */
        .collect::<Vec<Shift>>()
}

