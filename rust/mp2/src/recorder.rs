use csv::Writer;
use std::fmt::Result;
use std::fs::File;

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

pub struct NontrivialIterationResult {
    n: usize,
    pub dp_mem: Vec<(i32, i32)>,
    pub dp_tab: Vec<(i32, i32)>,
}

impl NontrivialIterationResult {
    pub fn new(n: usize) -> NontrivialIterationResult {
        NontrivialIterationResult {
            n,
            dp_mem: vec![],
            dp_tab: vec![],
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
                "Avg Time".to_string(),
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
                "Avg Time".to_string(),
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
                "Avg Time".to_string(),
                "Avg % Error".to_string(),
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
                "Avg % Error".to_string(),
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
                "Avg % Error".to_string(),
            ],
        );

        self.write_headers().unwrap();
        Ok(())
    }

    pub fn nontrivial_count_setup(&mut self) -> Result {
        self.add_header("n".to_string(), vec![]);

        self.add_header(
            "Dynamic Programming Memoization".to_string(),
            vec![
                "Computation 1".to_string(),
                "Retrieval 1".to_string(),
                "Computation 2".to_string(),
                "Retrieval 2".to_string(),
                "Computation 3".to_string(),
                "Retrieval 3".to_string(),
                "Average Computation".to_string(),
                "Average Retrieval".to_string(),
            ],
        );

        self.add_header(
            "Dynamic Programming Tabulation".to_string(),
            vec![
                "Computation 1".to_string(),
                "Retrieval 1".to_string(),
                "Computation 2".to_string(),
                "Retrieval 2".to_string(),
                "Computation 3".to_string(),
                "Retrieval 3".to_string(),
                "Average Computation".to_string(),
                "Average Retrieval".to_string(),
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

    // this is for the nontrivial data stuff, delete later if not needed
    pub fn add_int_data_to_row(&mut self, val: i32) {
        self.stored.push(val.to_string());
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

    pub fn write_nontrivial_iteration_result(
        &mut self,
        iteration_result: NontrivialIterationResult,
    ) -> Result {
        let mut to_write = vec![iteration_result.n.to_string()];

        let mut mem_sum_computations: u128 = 0;
        let mut mem_sum_retrievals: u128 = 0;
        for record in &iteration_result.dp_mem {
            to_write.push(record.0.to_string());
            to_write.push(record.1.to_string());

            mem_sum_computations += record.0 as u128;
            mem_sum_retrievals += record.1 as u128;
        }
        let avg_mem_computations: u128 =
            mem_sum_computations / iteration_result.dp_mem.len() as u128;
        let avg_mem_retrievals: u128 = mem_sum_retrievals / iteration_result.dp_mem.len() as u128;
        to_write.push(avg_mem_computations.to_string());
        to_write.push(avg_mem_retrievals.to_string());

        let mut tab_sum_computations: u128 = 0;
        let mut tab_sum_retrievals: u128 = 0;
        for record in &iteration_result.dp_tab {
            to_write.push(record.0.to_string());
            to_write.push(record.1.to_string());

            tab_sum_computations += record.0 as u128;
            tab_sum_retrievals += record.1 as u128;
        }

        let avg_tab_computations: u128 =
            tab_sum_computations / iteration_result.dp_tab.len() as u128;
        let avg_tab_retrievals: u128 = tab_sum_retrievals / iteration_result.dp_tab.len() as u128;
        to_write.push(avg_tab_computations.to_string());
        to_write.push(avg_tab_retrievals.to_string());

        self.writer.write_record(&to_write).unwrap();
        self.writer.flush().unwrap();
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
        let avg_time: u128 = sum_time / iteration_result.dp_mem.len() as u128;
        to_write.push(avg_time.to_string());

        // Prepare dynamic programming tabulation results + average
        let mut sum_time: u128 = 0;
        for record in &iteration_result.dp_tab {
            to_write.push(record.value.to_string());
            to_write.push(record.time.to_string());

            sum_time += record.time;
        }
        let avg_time: u128 = sum_time / iteration_result.dp_tab.len() as u128;
        to_write.push(avg_time.to_string());

        // Prepare greedy highest value results + average
        prepare_greedy_result(
            &iteration_result.greedy_highest_value,
            &iteration_result.dp_tab,
            &mut to_write,
        );

        // Prepare greedy smallest weight results + average
        prepare_greedy_result(
            &iteration_result.greedy_smallest_weight,
            &iteration_result.dp_tab,
            &mut to_write,
        );

        // Prepare greedy greatest ratio results + average
        prepare_greedy_result(
            &iteration_result.greedy_greatest_ratio,
            &iteration_result.dp_tab,
            &mut to_write,
        );

        // Write results to csv
        self.writer.write_record(&to_write).unwrap();
        self.writer.flush().unwrap();
        Ok(())
    }
}

fn prepare_greedy_result(
    greedy_records: &Vec<Record>,
    dp_records: &Vec<Record>,
    to_write: &mut Vec<String>,
) {
    let mut sum_time: u128 = 0;
    let mut sum_error: f64 = 0.0;

    for i in 0..greedy_records.len() {
        let greedy_record = &greedy_records[i];
        let dp_record = &dp_records[i];
        let error = (greedy_record.value as f64 - dp_record.value as f64) / dp_record.value as f64;

        to_write.push(greedy_record.value.to_string());
        to_write.push(greedy_record.time.to_string());

        sum_time += greedy_record.time;
        sum_error += error.abs();
    }

    let avg_time: u128 = sum_time / greedy_records.len() as u128;
    let avg_error: f64 = sum_error / greedy_records.len() as f64 * 100.0;

    to_write.push(avg_time.to_string());
    to_write.push(format!("{:.2}%", avg_error));
}
