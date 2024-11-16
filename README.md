## Usage

The script called **`./render-schedule`** implements a shift scheduling algorithm.
It is a `.exe` file compiled from Rust using [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html). To compile for yourself, install `cargo` and run `cargo build --release`.

The script must be run with the following arguments:
- **`--schedule`** - JSON file containing a definition of a schedule
- **`--overrides`** - JSON file containing an array of overrides
- **`--from`** - the time from which to start listing entries
- **`--until`** - the time until which to listing entries

There are two **optional** arguments:
- **`--pretty-print`** - flag used to print the schedule in a more user friendly format, rather than in JSON.
- **`--outfile, -O`** - path to an output file that the schedule is written to. The saved format is also determined by the `--pretty-print` flag.

The schedule will be truncated based on the `from`/`until` parameters. For example, if an entry was from 1pm November 17 -> 1pm November 19th, 
but `--from` was given as 2pm November 18th, the entry should instead be returned as 2pm November 18th -> 1pm November 19th.

### Help Information

Usage information is outputted if the command is run with the **`-h, --help`** flag.

```console
Scheduler for on-call shifts

Usage: render-schedule.exe [OPTIONS] --schedule <SCHEDULE_PATH> --overrides <OVERRIDE_PATH> --from <FROM> --until <UNTIL>

Options:
      --schedule <SCHEDULE_PATH>   Path to .json file containing schedule data - users, handover_start_date, handover_interval_days
      --overrides <OVERRIDE_PATH>  Path to .json file containing list of override shifts in priority descending order
      --from <FROM>                When rendered schedule should start
      --until <UNTIL>              When rendered schedule should end
  -p, --pretty-print               Pretty print the schedule to terminal
  -O, --outfile <OUT>              Optional path to an output file
  -h, --help                       Print help
  -V, --version                    Print version

```


## Example Usage

In JSON form, the configuration that describes how a schedule behaves might look like this:

`schedule.json`:
```js
// This is a schedule.
{
  "users": [
    "alice",
    "bob",
    "charlie"
  ],

  // 5pm, Friday 17th November 2023
  "handover_start_at": "2023-11-17T17:00:00Z",
  "handover_interval_days": 7
}
```

In that example, our schedule will rotate evenly between those users with the 
first shift starting at 5pm Friday 17th, with shift changes happening every 7 days.

That means:

- Alice takes the shift for 1 week, starting at 5pm, Friday 17th November
- Then Bob is on-call for 1 week from 5pm, Friday 24th November
- Then Charlie, then...
- Back to Alice again.

Visually, this might look like this:

![Schedule](./schedule.png)

### Overrides

Schedule systems often support **'overrides'** where you can add temporary shift modifications 
to a schedule, in case someone wants to go walk their dog or go to the cinema.

An override specifies the person that will take the shift and the time period it covers. 
An example of Charlie covering 5pm-10pm on Monday 20th November would look like this:

`overrides.json`:
```js
[
  // This is an override.
  {
    // Charlie will cover this shift
    "user": "charlie",
    // 5pm, Monday 20th November 2023
    "start_at": "2023-11-20T17:00:00Z",
    // 10pm, Monday 20th November 2023
    "end_at": "2023-11-20T22:00:00Z"
  }
]
```

### Command-Line Usage

To render the schedule, the program is run with the following command:

```console
$ ./render-schedule \
    --schedule=schedule.json \
    --overrides=overrides.json \
    --from='2023-11-17T17:00:00Z' \
    --until='2023-12-01T17:00:00Z'
```

### Output

It would then print the following to `stdout`:
```console
[
  {
    "user": "alice",
    "start_at": "2023-11-17T17:00:00Z",
    "end_at": "2023-11-20T17:00:00Z"
  },
  {
    "user": "charlie",
    "start_at": "2023-11-20T17:00:00Z",
    "end_at": "2023-11-20T22:00:00Z"
  },
  {
    "user": "alice",
    "start_at": "2023-11-20T22:00:00Z",
    "end_at": "2023-11-24T17:00:00Z"
  },
  {
    "user": "bob",
    "start_at": "2023-11-24T17:00:00Z",
    "end_at": "2023-12-01T17:00:00Z"
  }
]
```

Or with the `--pretty-print` flag set:

```console
alice                                              17:00 [17-11-2023] -> 17:00 [20-11-2023]
charlie                                            17:00 [20-11-2023] -> 22:00 [20-11-2023]
alice                                              22:00 [20-11-2023] -> 17:00 [24-11-2023]
bob                                                17:00 [24-11-2023] -> 17:00 [01-12-2023]
```

## Post-Mortem Thoughts

#### Initial Setup
I approached the problem by first getting the boilerplate command-line argument parsing and JSON loading completed. This was made easier using `CLAP`, `serde` and `chrono`, just a few of the well-supported crates available for Rust. This is one of the reasons I decided to use Rust, the others being its support for error-handling and functional programming, which facilitated list traversal through the schedule.

#### Override Handling
Having created the Schedule and Shift structs, I began handling the generation of the initial shift schedule. To apply the overrides, I mutated the original schedule after its generation. I initially tried to find which shifts should be overridden by calculating the index from the schedule's shift length. Once the override is inserted, the shifts are stale and needed to be updated. Since overrides have no guaranteed set length, however, an indexing operation could no longer be performed.
I decided to implement my own binary search to find each shift to override, exploiting the sorted property of the schedule for fast lookups. This allowed me to apply the overrides in one shot and ensured no index was stale.

Ensuring the overrides were consistent and applied correctly was difficult, requiring lots of testing to try and handle most cases; it was important for me to be permissive and allow the user to enter conflicting overrides, handling them in the program. This means the admin can add overrides as they require easily without being forced to double-check themselves.

#### Further Development
I attempted to build a full-fledged scheduler with options for pretty printing and outputting to a file. I wanted the program to be as user-friendly as possible. A command-line program is limited, however; if I were to extend the program, I would support a graphics interface with a more interactive input system for the schedule and overrides. 

A complementary system to this could be an online system that allows employees to easily request time off without having to go through formal channels. This would encourage users to keep a healthy work-life balance while avoiding the stress of a one-on-one meeting with a supervisor. The supervisor can then simply allocate the user to the override and regenerate the schedule.