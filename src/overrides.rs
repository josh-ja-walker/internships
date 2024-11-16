use crate::{helpers, scheduler::Shift};


/* Apply overrides to shift schedule */
pub fn apply_overrides(shifts: &mut Vec<Shift>, overrides: Vec<Shift>) {
    if shifts.is_empty() || overrides.is_empty() {
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
    
    // Do not modify existing shifts if the shift index cannot be found
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

    /* Remove all entirely overridden shifts */
    shifts.drain((prev_shift.unwrap_or(0) + 1)..post_shift.unwrap_or(shifts.len() - 1));

    /* Insert override - ensure override is inserted after the previous shift or at 0 if prev is None */
    shifts.insert(prev_shift.map_or(0, |i| i + 1), override_shift);
}


