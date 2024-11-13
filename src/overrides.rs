use crate::scheduler::{find_shift_index, Shift};


/* Apply overrides to shift schedule */
pub fn apply_overrides(shifts: &mut Vec<Shift>, overrides: Vec<Shift>) {
    /* List of indices of shifts before and after each override */
    let mut shift_idxs: Vec<(Option<usize>, Option<usize>)> = vec![];

    /* Populate shift_idxs with correct indices */
    for override_shift in &overrides {
        let prev_index: Option<usize> = find_shift_index(override_shift.start_at(), &shifts);
        let post_index: Option<usize> = find_shift_index(override_shift.end_at(), &shifts);

        shift_idxs.push((prev_index, post_index));
    }

    /* Add each override, modifying previous and following shifts */
    for ((prev_shift, post_shift), override_shift) in shift_idxs.iter().zip(overrides) {
        apply_override(*prev_shift, override_shift, *post_shift, shifts);
    }
}

/* Apply single override */
fn apply_override(prev_shift: Option<usize>, override_shift: Shift, post_shift: Option<usize>, shifts: &mut Vec<Shift>) {
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
    } else {
        /* Otherwise remove all entirely overridden shifts */
        shifts.drain((clamped_prev + 1)..clamped_post);
    }
    
    /* Modify existing shifts */
    shifts[clamped_prev].update_end_at(override_shift.start_at()); /* End previous shift earlier */
    shifts[clamped_post].update_start_at(override_shift.end_at()); /* Start next shift later */


    /* Insert override - ensure override is inserted after the previous shift or at 0 if prev is None */
    shifts.insert(prev_shift.map_or(0, |i| i + 1), override_shift);
}


