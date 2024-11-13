use std::{fs, io, path::PathBuf};

use chrono::{DateTime, Utc};
use clap::Parser;
use scheduler::{Schedule, Shift};

mod scheduler;
mod overrides;


/// Scheduler for on-call shifts
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to .json file containing schedule data - users, handover_start_date, handover_interval_days 
    #[arg(long="schedule")]
    schedule_path: PathBuf,
    
    /// Path to .json file containing list of override shifts in priority descending order
    #[arg(long="overrides")]
    override_path: PathBuf,
    
    /// When rendered schedule should start 
    #[arg(long)]
    from: DateTime<Utc>,
    
    /// When rendered schedule should end 
    #[arg(long)]
    until: DateTime<Utc>,
}


fn main() -> io::Result<()> { 
	let args = Args::parse(); /* Parse command-line arguments */
    
    /* Parse schedule struct from .json file */
    let schedule: Schedule = serde_json::from_str(&fs::read_to_string(args.schedule_path)?)?;
    
    /* Parse list of overrides from .json file */
    let mut overrides: Vec<Shift> = serde_json::from_str(&fs::read_to_string(args.override_path)?)?;
    overrides.reverse(); /* Assumed overrides are in descending priority order - reverse so most important override applied last */

    /* Schedule the shifts using schedule data and overrides, filtering to within from - until */
    let shifts: Vec<Shift> = scheduler::schedule_shifts(schedule, overrides, args.from, args.until);

    println!("{}", serde_json::to_string_pretty(&shifts)?);

    Ok(())
}
