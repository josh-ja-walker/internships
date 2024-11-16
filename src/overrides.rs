use crate::{helpers, scheduler::Shift};


/* Apply overrides to shift schedule */
pub fn apply_overrides(shifts: &mut Vec<Shift>, overrides: Vec<Shift>) {
    if shifts.is_empty() || overrides.is_empty() {
        return;
    }
    
    /* Populate shift_idxs with correct indices */
    for override_shift in overrides {
        apply_override(override_shift, shifts);
    }
}

/* Apply single override */
fn apply_override(override_shift: Shift, shifts: &mut Vec<Shift>) {
    let prev_shift: Option<usize> = helpers::find_shift_index(override_shift.start_at(), &shifts);
    let post_shift: Option<usize> = helpers::find_shift_index(override_shift.end_at(), &shifts);
    
    /* Helper function to unpack and clamp index of shift to avoid index out of range errors */
    let clamp_idx = |idx: Option<usize>| (idx.unwrap_or(0)).clamp(0, shifts.len() - 1);
    
    /* Clamp prev and post shift indices */
    let clamped_prev: usize = clamp_idx(prev_shift); 
    let mut clamped_post: usize = clamp_idx(post_shift);

    /* If override is contained within a single shift */
    if clamped_prev == clamped_post {
        /* Duplicate the shift to create post shift */
        clamped_post = clamped_prev + 1;
        shifts.insert(clamped_post, shifts[clamped_prev].clone());
    }
    
    /* Modify existing shifts */
    shifts[clamped_prev].set_end_at(override_shift.start_at()); /* End previous shift earlier */
    shifts[clamped_post].set_start_at(override_shift.end_at()); /* Start next shift later */

    /* Remove all entirely overridden shifts */
    shifts.drain((clamped_prev + 1)..clamped_post);

    /* Insert override - ensure override is inserted after the previous shift or at 0 if prev is None */
    shifts.insert(prev_shift.map_or(0, |i| i + 1), override_shift);
}


