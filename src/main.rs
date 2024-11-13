use std::{io, path::PathBuf};

use chrono::{DateTime, Utc};
use clap::Parser;

/// Scheduler for on-call shifts
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to .json file containing schedule data - users, handover_start_date, handover_interval_days 
    #[arg(long="schedule")]
    schedule_path: PathBuf,
    
    /// Path to .json file containing list of override shifts 
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
	let args = Args::parse();
    println!("{args:#?}");
    Ok(())
}
