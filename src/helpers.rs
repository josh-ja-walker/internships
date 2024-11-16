use chrono::{DateTime, Utc};

use crate::scheduler::{Schedule, Shift};


/* Filter and truncate shifts not include in from - until timeframe */
pub fn truncate_shifts(shifts: Vec<Shift>, schedule: &Schedule, from: DateTime<Utc>, until: DateTime<Utc>) -> Vec<Shift> {
    /* Filter out shifts which are completely disjoint with from - until timeframe */
    let mut trunc_shifts: Vec<Shift> = shifts.into_iter()
        .filter(|shift: &Shift| shift.end_at() > from && shift.start_at() < until) 
        .collect();
    
    /* Truncate overflowing head and tail */
    if !trunc_shifts.is_empty() {
        /* Do not move first shift's start earlier than handover_start_at */
        if from > schedule.handover_start_at() {
            trunc_shifts.first_mut().unwrap().set_start_at(from);
        }
        
        /* Truncate last shift's end time to until */
        trunc_shifts.last_mut().unwrap().set_end_at(until);
    
        /* This cannot create an invalid shift (unless from >= until) because 
            shift end > from and shift end < until so shift start < shift end */
    }
    
    /* Filter out invalid shifts */
    trunc_shifts.into_iter()
        .filter(|shift: &Shift| shift.is_valid()) 
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

