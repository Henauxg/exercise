use std::io::{self, BufRead};

fn main() {
    let (valid_count, invalid_count) = parse_csv_from_stdin();

    println!("{valid_count} {invalid_count}");
}

fn parse_csv_from_stdin() -> (usize, usize) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse CSV headers
    let Some(Ok(headers_line)) = lines.next() else {
        return (0, 0);
    };
    let headers: Vec<&str> = headers_line.split(',').collect();
    // Find the "ean" header
    let Some(ean_column_index) = headers.iter().position(|c| *c == "ean") else {
        // TODO missing_ean_column expects invalid count to be the number of lines in the file
        return (0, lines.count() + 1);
    };

    let mut valid_count = 0;
    let mut invalid_count = 0;

    // TODO Parse content
    while let Some(line) = lines.next() {
        let Ok(line) = line else {
            // TODO Should it be considered an error on the whole file ?
            invalid_count += 1;
            continue;
        };
        let columns: Vec<&str> = line.split(',').collect();
        // TODO Could/should consider that missing columns (not just EAN) is an error ?
        let Some(_ean) = columns.get(ean_column_index) else {
            invalid_count += 1;
            continue;
        };

        // TODO Verify EAN
        valid_count += 1;
    }

    (valid_count, invalid_count)
}
