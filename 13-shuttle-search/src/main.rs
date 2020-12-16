fn find_next_bus(start_time: u64, buses: &Vec<u64>) -> (u64, u64) {
    let mut t = start_time;
    loop {
        for bus_num in buses.iter() {
            // println!("{} % {} => {}", t, bus_num, t % bus_num);
            if t % bus_num == 0 {
                return (t, *bus_num);
            }
        }
        t += 1;
    }
}

fn find_consecutive_departures(buses: &Vec<Option<u64>>) -> u64 {
    // check each bus in sequence.  Once we have found a match, we can
    // go one tick forward and then step by that bus's number to go
    // directly to the next time just after it departs and check if the
    // next bus departs then.  Once we have found a time when the first
    // n conditions are satisfied, we can step by the product of the
    // first n bus's numbers because the pattern will repeat with that
    // period, and look for a match for the n+1th bus.

    let mut step = 1;
    let mut t = 1;
    for bus in buses {
        // println!("matching {:?}", bus);

        match bus {
            Some(n) => {
                while t % n != 0 {
                    t += step;
                }
                step *= n;
                t += 1;
            },
            None => { t += 1; }
        }
    }

    // now we are at the end of the consecutive departure chain, and
    // need to go back to the beginning.
    return t - (buses.len() as u64);
}

fn is_consecutive_at_timestamp(buses: &Vec<Option<u64>>, time: u64) -> bool {
    for (bus, i) in buses.iter().zip(0u64..(buses.len() as u64)) {
        match bus {
            Some(n) => { if (time + i) % n != 0 { return false }},
            None => {}
        }
    }
    return true;
}

fn main() {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut lines = input.lines();
    let current_time: u64 = lines.next().unwrap().parse().unwrap();

    let bus_numbers = lines.next().unwrap()
        .split(',')
        .map(|s| match s {
            "x" => None,
            _ => Some(s.parse().unwrap())
        })
        .collect::<Vec<Option<u64>>>();

    let consecutive_timestamp = find_consecutive_departures(&bus_numbers);

    println!("{}", consecutive_timestamp);
}
