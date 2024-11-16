use std::{fs, io, path::PathBuf};

use chrono::{DateTime, Utc};
use clap::Parser;
use scheduler::{Schedule, Shift};

mod scheduler;
mod helpers;
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
    
    /// Pretty print the schedule to terminal
    #[arg(short, long, default_value_t = false)]
    pretty_print: bool,

    /// Optional path to an output file
    #[arg(short='O', long="outfile")]
    out: Option<PathBuf>,
}


impl Args {
    /* Print warnings about command-line arguments passed */
    fn print_warnings(&self) {
        if self.from > self.until {
            println!("Warning! --from={:?} is after --until={:?}", self.until, self.from);
        }

        Args::path_warnings("schedule", &self.schedule_path);
        Args::path_warnings("overrides", &self.override_path);
    }

    /* Print warnings about invalid or empty json files */
    fn path_warnings(arg_name: &'static str, path: &PathBuf) {
        if let Ok(s) = fs::read_to_string(&path) {
            if s.is_empty() {
                println!("Warning! --{arg_name}={:?} file is empty", path);
            }
        } else {
            println!("Warning! --{arg_name}={:?} file cannot be found", path);
        }
    }

    /* Print warnings about inconsistent schedule start date and from - until arguments */
    fn schedule_warnings(&self, schedule: &Schedule) {
        let warning = |date: DateTime<Utc>, name: &'static str| 
            if schedule.handover_start_at() > date {
                println!("Warning! --{name}={:?} is before schedule start date {:?}", self.from, schedule.handover_start_at());
            };
        
        warning(self.from, "from");
        warning(self.until, "until");
    }

    /* Consume and destruct args struct, returning usable information */
    fn unpack(self) -> io::Result<(Schedule, Vec<Shift>, DateTime<Utc>, DateTime<Utc>)> {
        self.print_warnings();
        
        /* Parse schedule struct from .json file */
        let schedule: Schedule = serde_json::from_str(&fs::read_to_string(&self.schedule_path)?)?;
        self.schedule_warnings(&schedule);
    
        /* Parse list of overrides from .json file */
        let mut overrides: Vec<Shift> = serde_json::from_str(&fs::read_to_string(&self.override_path)?)?;
        overrides.reverse(); /* Assumed overrides are in descending priority order - reverse so most important override applied last */
    
        Ok((schedule, overrides, self.from, self.until))
    }
}


/* Schedule all shifts, overrides included */
fn perform_scheduling(schedule: Schedule, overrides: Vec<Shift>, from: DateTime<Utc>, until: DateTime<Utc>) -> Vec<Shift> {
    /* Schedule the shifts using schedule data */
    let mut shifts: Vec<Shift> = scheduler::schedule_shifts(&schedule, until);
    
    overrides::apply_overrides(&mut shifts, overrides); /* Override existing shifts */
    helpers::truncate_shifts(shifts, &schedule, from, until) /* Filter and truncate shifts to [from, until] */
}


fn main() -> io::Result<()> {
	let args = Args::parse(); /* Parse command-line arguments */

    let pretty_print: bool = args.pretty_print; /* Copy pretty_print value before destructing args */
    let outfile: Option<PathBuf> = args.out.clone(); /* Copy out value before destructing args */
    let (schedule, overrides, from, until) = args.unpack()?; /* Destruct args and deserialise json files */
    
    /* Perform scheduling algorithm */
    let shifts: Vec<Shift> = perform_scheduling(schedule, overrides, from, until); 

    /* If no shifts are scheduled, return */
    if shifts.is_empty() {
        println!("Warning! No shifts scheduled");
        return Ok(());
    }

    let out = if !pretty_print {
        /* Pretty print serialised json */
        serde_json::to_string_pretty(&shifts)?
    } else {
        /* Pretty print using Shift formatter */
        shifts.into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    };

    /* Print schedule to stdout */
    println!("{}", out);

    /* Write to output file */
    if let Some(path) = outfile {
        println!("Schedule saved as '{}'", path.display());
        return fs::write(path, out);
    }

    Ok(())
}
