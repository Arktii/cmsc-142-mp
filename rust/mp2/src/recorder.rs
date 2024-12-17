use csv::Writer;
use std::{fmt::Result, fs::File};

pub struct Record {
    value: usize,
    time: u128,
}

impl Record {
    pub fn new(value: usize, time: u128) -> Record {
        Record { value, time }
    }
}

pub struct IterationResult {
    n: usize,
    pub dp_mem: Vec<Record>,
    pub dp_tab: Vec<Record>,
    pub greedy_highest_value: Vec<Record>,
    pub greedy_smallest_weight: Vec<Record>,
    pub greedy_greatest_ratio: Vec<Record>,
}

impl IterationResult {
    pub fn new(n: usize) -> IterationResult {
        IterationResult {
            n,
            dp_mem: vec![],
            dp_tab: vec![],
            greedy_highest_value: vec![],
            greedy_smallest_weight: vec![],
            greedy_greatest_ratio: vec![],
        }
    }
}

struct Header {
    name: String,
    columns: Vec<String>,
}

pub struct Recorder {
    writer: Writer<File>,
    headers: Vec<Header>,
    stored: Vec<String>,
}

impl Recorder {
    // results/results.csv
    pub fn new(path: &str) -> Recorder {
        let file = File::create(path).unwrap();

        Recorder {
            writer: Writer::from_writer(file),
            headers: vec![],
            stored: vec![],
        }
    }

    pub fn default_setup(&mut self) -> Result {
        self.add_header("n".to_string(), vec![]);
        self.add_header(
            "Dynamic Programming Memoization".to_string(),
            vec![
                "Value 1".to_string(),
                "Time 1".to_string(),
                "Value 2".to_string(),
                "Time 2".to_string(),
                "Value 3".to_string(),
                "Time 3".to_string(),
                "Average Time".to_string(),
            ],
        );

        self.add_header(
            "Dynamic Programming Tabulation".to_string(),
            vec![
                "Value 1".to_string(),
                "Time 1".to_string(),
                "Value 2".to_string(),
                "Time 2".to_string(),
                "Value 3".to_string(),
                "Time 3".to_string(),
                "Average Time".to_string(),
            ],
        );

        self.add_header(
            "Greedy Highest Value".to_string(),
            vec![
                "Value 1".to_string(),
                "Time 1".to_string(),
                "Value 2".to_string(),
                "Time 2".to_string(),
                "Value 3".to_string(),
                "Time 3".to_string(),
                "Average Time".to_string(),
            ],
        );

        self.add_header(
            "Greedy Smallest Weight".to_string(),
            vec![
                "Value 1".to_string(),
                "Time 1".to_string(),
                "Value 2".to_string(),
                "Time 2".to_string(),
                "Value 3".to_string(),
                "Time 3".to_string(),
                "Average Time".to_string(),
            ],
        );

        self.add_header(
            "Greedy Greatest Worth Ratio".to_string(),
            vec![
                "Value 1".to_string(),
                "Time 1".to_string(),
                "Value 2".to_string(),
                "Time 2".to_string(),
                "Value 3".to_string(),
                "Time 3".to_string(),
                "Average Time".to_string(),
            ],
        );

        self.write_headers().unwrap();
        Ok(())
    }

    pub fn add_header(&mut self, name: String, columns: Vec<String>) {
        self.headers.push(Header { name, columns });
    }

    pub fn write_headers(&mut self) -> Result {
        let mut line_1 = vec![];
        let mut line_2 = vec![];

        for header in &self.headers {
            line_1.push(header.name.to_string());

            if header.columns.len() == 0 {
                line_2.push("".to_string());
            } else {
                line_2.push(header.columns[0].to_string());
            }

            for i in 1..header.columns.len() {
                line_1.push("".to_string());
                line_2.push(header.columns[i].to_string());
            }
        }

        self.writer.write_record(line_1).unwrap();
        self.writer.write_record(line_2).unwrap();
        self.writer.flush().unwrap();
        Ok(())
    }

    pub fn clear_stored(&mut self) {
        self.stored.clear();
    }

    pub fn add_record_to_row(&mut self, value: usize, time: u128) {
        self.stored.push(value.to_string());
        self.stored.push(time.to_string());
    }

    pub fn add_to_row(&mut self, item: String) {
        self.stored.push(item);
    }

    // Write stored data to csv and clear stored
    pub fn write_row(&mut self) -> Result {
        self.writer.write_record(&self.stored).unwrap();

        self.clear_stored();

        Ok(())
    }

    pub fn write_iteration_result(&mut self, iteration_result: IterationResult) -> Result {
        let mut to_write = vec![iteration_result.n.to_string()];

        // Prepare dynamic programming memoization results + average
        let mut sum_time: u128 = 0;
        for record in &iteration_result.dp_mem {
            to_write.push(record.value.to_string());
            to_write.push(record.time.to_string());

            sum_time += record.time;
        }
        let avg_time: f64 = sum_time as f64 / iteration_result.dp_mem.len() as f64;
        to_write.push(avg_time.to_string());

        // Prepare dynamic programming tabulation results + average
        let mut sum_time: u128 = 0;
        for record in &iteration_result.dp_tab {
            to_write.push(record.value.to_string());
            to_write.push(record.time.to_string());

            sum_time += record.time;
        }
        let avg_time: f64 = sum_time as f64 / iteration_result.dp_tab.len() as f64;
        to_write.push(avg_time.to_string());

        // TODO: include error percentage, since either of the dp should be exact and correct

        // Prepare greedy highest value results + average
        let mut sum_time: u128 = 0;
        for record in &iteration_result.greedy_highest_value {
            to_write.push(record.value.to_string());
            to_write.push(record.time.to_string());

            sum_time += record.time;
        }
        let avg_time: f64 = sum_time as f64 / iteration_result.greedy_highest_value.len() as f64;
        to_write.push(avg_time.to_string());

        // Prepare greedy highest value results + average
        let mut sum_time: u128 = 0;
        for record in &iteration_result.greedy_smallest_weight {
            to_write.push(record.value.to_string());
            to_write.push(record.time.to_string());

            sum_time += record.time;
        }
        let avg_time: f64 = sum_time as f64 / iteration_result.greedy_smallest_weight.len() as f64;
        to_write.push(avg_time.to_string());

        // Prepare greedy highest value results + average
        let mut sum_time: u128 = 0;
        for record in &iteration_result.greedy_greatest_ratio {
            to_write.push(record.value.to_string());
            to_write.push(record.time.to_string());

            sum_time += record.time;
        }
        let avg_time: f64 = sum_time as f64 / iteration_result.greedy_greatest_ratio.len() as f64;
        to_write.push(avg_time.to_string());

        // Write results to csv
        self.writer.write_record(&to_write).unwrap();
        self.writer.flush().unwrap();
        Ok(())
    }
}

// fn write_to_csv(wtr: &mut Writer<File>, n: usize, iteration_data: &[Record]) {
//     let values: Vec<String> = iteration_data
//         .iter()
//         .flat_map(|record| vec![record.value.to_string(), record.time.to_string()])
//         .collect();

//     let sum_time: u128 = iteration_data.iter().map(|r| r.time).sum();
//     let avg_time: f64 = sum_time as f64 / iteration_data.len() as f64;

//     let mut record = vec![n.to_string()];
//     record.extend(values);
//     record.push(avg_time.to_string());

//     wtr.write_record(&record).unwrap();
//     wtr.flush().unwrap();
// }
