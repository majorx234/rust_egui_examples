#[macro_use]
extern crate scan_fmt;

pub fn read_data() -> (usize, std::vec::Vec<f32>) {
    let mut line = String::new();
    let line_size = std::io::stdin().read_line(&mut line).unwrap();
    let num_samples_raw = scan_fmt!(&line, "{}\n", usize);
    let num_samples: usize = num_samples_raw.unwrap();

    let mut values_data = Vec::new();
    for _ in 0..num_samples {
        let mut line = String::new();
        let line_size = std::io::stdin().read_line(&mut line).unwrap();
        let sample = scan_fmt!(&line, "{}\n", f32);

        values_data.push(sample.unwrap());
    }
    return (num_samples, values_data);
}
