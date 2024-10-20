use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rust_xlsxwriter::{Workbook, Worksheet, Format, FormatAlign};

const MAX_N: usize = 32;
const KNAPSACK_CAPACITY: i32 = 1000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n_start: usize = args[1]
        .parse()
        .expect("Please provide n as a command-line argument e.g. cargo run -- 25");

    if n_start > MAX_N {
        panic!("n cannot be greater than the number of items ({}).", MAX_N);
    }

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    create_worksheet_headers(worksheet);

    for n in n_start..=MAX_N {
        for i in 0..3 {
            let items = ITEM_SETS[i].to_vec();

            let start = Instant::now();

            let (solution, solution_weight, solution_value) = brgc_knapsack(&items, n);

            let duration = start.elapsed();

            let filename = format!("outputs/output_{}_{}.txt", n, i + 1);
            let mut file = File::create(filename).expect("Unable to create file");

            // Write everything to file
            writeln!(file, "Item set {}:", i + 1).unwrap();
            write!(file, "Solution: ").unwrap();
            write_solution(&mut file, &solution);
            writeln!(file, "\nWeight: {}", solution_weight).unwrap();
            writeln!(file, "Value: {}", solution_value).unwrap();
            writeln!(file, "Total time: {:?}", duration).unwrap();

            // duration is in microsecs
            record_data(worksheet, n as i32, i + 1, duration.as_micros() as f64, &solution, solution_weight, solution_value, (n - n_start + 2) as u32);

            // println!("\nItem set {}:", i + 1);
            // print!("Solution: ");
            // print_solution(&solution);
            // println!();
            // println!("Weight: {}", solution_weight);
            // println!("Value: {}", solution_value);

            // println!("Total time: {:?}", duration);
        }
        /*
            idk how to do this
            https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.save
            says there that save() can be called multiple times or maybe i dont understand it correctly
         */
        // let xlsx_file = format!("outputs/Results_{}_to_{}.xlsx", n_start, MAX_N);
        // let _ = workbook.save(&xlsx_file);
    }
    let xlsx_file = format!("outputs/Results_{}_to_{}.xlsx", n_start, MAX_N);
    let _ = workbook.save(&xlsx_file);
}

fn brgc_knapsack(items: &Vec<(i32, i32)>, n: usize) -> (Vec<bool>, i32, i32) {
    let mut solution = vec![false; n];
    let mut solution_weight = 0;
    let mut solution_value = 0;

    let mut current = vec![false; n];
    let mut current_weight = 0;
    let mut current_value = 0;

    for i in 1..((1 as u64) << n) {
        let change_index = get_index_to_flip(&i);

        current[change_index] = !current[change_index];

        if current[change_index] {
            current_weight += items[change_index].0;
            current_value += items[change_index].1;
        } else {
            current_weight -= items[change_index].0;
            current_value -= items[change_index].1;
        }

        if current_weight <= KNAPSACK_CAPACITY && current_value > solution_value {
            solution = current.clone();
            solution_weight = current_weight;
            solution_value = current_value;
        }
    }

    (solution, solution_weight, solution_value)
}

fn get_index_to_flip(i: &u64) -> usize {
    i.trailing_zeros() as usize
}

// fn print_solution(solution: &Vec<bool>) {
//     print!("[");
//     for i in 0..solution.len() - 1 {
//         print!("{}", if solution[i] { 1 } else { 0 });
//         print!(", ");
//     }
//     print!("{}", if solution[solution.len() - 1] { 1 } else { 0 });
//     print!("]");
// }

fn write_solution(file: &mut File, solution: &Vec<bool>) {
    write!(file, "[").unwrap();
    for i in 0..solution.len() - 1 {
        write!(file, "{}, ", if solution[i] { 1 } else { 0 }).unwrap();
    }
    write!(file, "{}", if solution[solution.len() - 1] { 1 } else { 0 }).unwrap();
    write!(file, "]").unwrap();
}

fn create_worksheet_headers(worksheet: &mut Worksheet) {
    let format = Format::new().set_bold().set_align(FormatAlign::Center).set_align(FormatAlign::VerticalCenter);

    let _ = worksheet.set_column_width(0, 16); //N column
    let _ = worksheet.merge_range(0, 0, 1, 0, "N", &format) ;

    for (index, i) in [1, 5, 9].iter().enumerate() {
        let _ =worksheet.set_column_width(*i, 16);
        let _ =worksheet.set_column_width(*i + 1, 16);
        let _ =worksheet.set_column_width(*i + 2, 8);
        let _ =worksheet.set_column_width(*i + 3, 8);

        let trial = format!("Trial {}", index + 1);

        let _ = worksheet.merge_range(0, *i, 0, *i + 3, &trial, &format) ;
        let _ = worksheet.write(1, *i, "Time");
        let _ = worksheet.write(1, *i + 1, "Solution");
        let _ = worksheet.write(1, *i + 2, "Weight");
        let _ = worksheet.write(1, *i + 3, "Value");
    }
}

fn record_data(worksheet: &mut Worksheet, n: i32, trial: usize, time: f64, solution: &Vec<bool>, solution_weight: i32, solution_value: i32, row: u32) {
    let col_num: u16 = ((trial - 1) * 4) as u16;

    let _ = worksheet.write(row, 0, n);
    let _ = worksheet.write(row, col_num + 1, time);
    let solution_str: String = solution.iter().map(|&s| if s { '1' } else { '0' }).collect();
    let _ = worksheet.write(row, col_num + 2, &solution_str);
    let _ = worksheet.write(row, col_num + 3, solution_weight);
    let _ = worksheet.write(row, col_num + 4, solution_value);

    // I think it looks nicer if the data is centered
    let center_format = Format::new().set_align(FormatAlign::Center).set_align(FormatAlign::VerticalCenter);
    let _ = worksheet.set_row_format(row, &center_format);
}

const ITEM_SETS: [[(i32, i32); 50]; 3] = [
    [
        (51, 212),
        (50, 402),
        (62, 395),
        (59, 150),
        (64, 289),
        (65, 373),
        (68, 267),
        (92, 376),
        (79, 173),
        (68, 451),
        (59, 135),
        (96, 355),
        (96, 306),
        (80, 247),
        (55, 179),
        (71, 493),
        (81, 286),
        (88, 124),
        (82, 414),
        (74, 235),
        (87, 342),
        (68, 125),
        (96, 262),
        (84, 347),
        (76, 118),
        (82, 232),
        (91, 296),
        (84, 406),
        (50, 175),
        (61, 371),
        (50, 420),
        (89, 424),
        (97, 407),
        (63, 492),
        (79, 313),
        (85, 443),
        (66, 416),
        (89, 199),
        (85, 139),
        (94, 271),
        (97, 352),
        (93, 460),
        (69, 112),
        (55, 334),
        (72, 471),
        (53, 157),
        (96, 408),
        (90, 433),
        (60, 432),
        (70, 197),
    ],
    [
        (72, 172),
        (55, 236),
        (66, 397),
        (98, 262),
        (99, 439),
        (68, 400),
        (93, 488),
        (79, 458),
        (72, 451),
        (68, 123),
        (51, 204),
        (84, 269),
        (80, 185),
        (63, 167),
        (75, 312),
        (78, 403),
        (54, 468),
        (61, 273),
        (71, 451),
        (55, 423),
        (93, 362),
        (91, 194),
        (100, 181),
        (68, 218),
        (72, 278),
        (77, 161),
        (64, 320),
        (87, 174),
        (95, 381),
        (95, 298),
        (92, 387),
        (61, 258),
        (54, 243),
        (65, 311),
        (73, 234),
        (77, 195),
        (98, 479),
        (97, 270),
        (74, 142),
        (80, 231),
        (99, 427),
        (66, 432),
        (88, 158),
        (94, 453),
        (53, 346),
        (76, 467),
        (54, 138),
        (66, 429),
        (70, 202),
        (96, 158),
    ],
    [
        (59, 186),
        (60, 420),
        (92, 324),
        (70, 300),
        (60, 494),
        (100, 133),
        (61, 204),
        (94, 490),
        (99, 119),
        (60, 384),
        (54, 347),
        (63, 164),
        (100, 237),
        (67, 456),
        (62, 136),
        (96, 133),
        (65, 397),
        (54, 103),
        (63, 387),
        (62, 184),
        (74, 373),
        (83, 212),
        (52, 496),
        (78, 342),
        (76, 232),
        (75, 169),
        (82, 148),
        (50, 116),
        (62, 424),
        (72, 228),
        (95, 326),
        (52, 327),
        (78, 231),
        (89, 326),
        (73, 485),
        (75, 404),
        (55, 248),
        (55, 175),
        (61, 172),
        (61, 181),
        (89, 377),
        (72, 246),
        (79, 277),
        (76, 289),
        (80, 474),
        (96, 159),
        (71, 339),
        (93, 280),
        (52, 476),
        (77, 149),
    ],
];
