fn find_next_bus(start_time: i32, buses: &Vec<i32>) -> (i32, i32) {
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

fn main() {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut lines = input.lines();
    let current_time: i32 = lines.next().unwrap().parse().unwrap();

    let bus_numbers = lines.next().unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let (first_bus_time, first_bus_num) = find_next_bus(current_time, &bus_numbers);

    println!("{}", (first_bus_time - current_time) * first_bus_num);
}
