use chrono::{DateTime, Utc};

use crate::{helpers, scheduler::Shift};


/* Apply overrides to shift schedule */
pub fn apply_overrides(shifts: &mut Vec<Shift>, overrides: Vec<Shift>) {
    if overrides.is_empty() {
        return;
    }

    /* Apply each override in turn (assume overrides are given in ascending priority order) */
    for override_shift in overrides {
        apply_override(override_shift, shifts);
    }
}

/* Apply single override */
fn apply_override(override_shift: Shift, shifts: &mut Vec<Shift>) {
    let prev_shift: Option<usize> = helpers::find_shift_index(override_shift.start_at(), &shifts);
    let mut post_shift: Option<usize> = helpers::find_shift_index(override_shift.end_at(), &shifts);
    
    /* Do not modify existing shifts if the shift index cannot be found */
    if let (Some(prev), Some(post)) = (prev_shift, post_shift) {
        /* Helper function to unpack and clamp index of shift to avoid index out of range errors */
        
        /* If override is contained within a single shift */
        if prev == post {
            /* Duplicate the shift to create post shift */
            post_shift = Some(prev + 1);
            shifts.insert(post, shifts[prev].clone());
        } 
    }

    /* Modify existing shifts */
    prev_shift.map(|i| shifts[i].set_end_at(override_shift.start_at())); /* End previous shift earlier */
    post_shift.map(|j| shifts[j].set_start_at(override_shift.end_at())); /* Start next shift later */

    let handle_none = |found_idx: Option<usize>, time: DateTime<Utc>| match found_idx {
        /* Shift was found, return */
        Some(x) => x,
        
        /* Time is before all shifts (or shifts empty), clamp to 0  */
        None if shifts.is_empty() || time < shifts.first().unwrap().start_at() => 0, 
        
        /* Time is after all shifts, clamp to last index */
        None => shifts.len() - 1, 
    };
    
    let clamped_prev = handle_none(prev_shift, override_shift.start_at());
    let clamped_post = handle_none(post_shift, override_shift.end_at());
    
    /* Remove all entirely overridden shifts, checking to avoid slice errors */
    if clamped_prev < clamped_post {
        shifts.drain((clamped_prev + 1)..clamped_post);
    }

    /* If previous shift not found and clamped to 0, should insert at 0 */
    let insert_at = if let (None, 0) = (prev_shift, clamped_prev) { 0 } else { clamped_prev + 1 };

    /* Insert override - ensure override is inserted after the previous shift or at 0 if prev is None */
    shifts.insert(insert_at, override_shift);
}


