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
    let ean_column_index = match headers.iter().position(|c| *c == "ean") {
        Some(column_index) => column_index,
        // "If the header is missing, it will be the first column"
        None => 0,
    };

    let mut valid_count = 0;
    let mut invalid_count = 0;

    // Parse content
    while let Some(line) = lines.next() {
        let Ok(line) = line else {
            // TODO Should it be considered an error on the whole file ?
            invalid_count += 1;
            continue;
        };
        if line.is_empty() {
            // "If the line is empty, skip it"
            continue;
        }
        let columns: Vec<&str> = line.split(',').collect();
        let Some(ean_str) = columns.get(ean_column_index) else {
            invalid_count += 1;
            continue;
        };
        // "Quoted EANs should be considered as valid if the EAN itself is"
        let ean = ean_str.trim_matches('"');

        match is_valid_gtin_13(ean) {
            true => valid_count += 1,
            false => invalid_count += 1,
        };
    }

    (valid_count, invalid_count)
}

fn is_valid_gtin_13(_ean: &str) -> bool {
    // TODO
    false
}

fn gtin_13_checksum(ean: Vec<u8>) -> Option<u8> {
    if ean.len() < 12 {
        return None;
    }
    let sum = ean[0]
        + 3 * ean[1]
        + ean[2]
        + 3 * ean[3]
        + ean[4]
        + 3 * ean[5]
        + ean[6]
        + 3 * ean[7]
        + ean[8]
        + 3 * ean[9]
        + ean[10]
        + 3 * ean[11];

    let checksum = match sum % 10 {
        0 => 0,
        n => 10 - n,
    };

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksums() {
        let result = gtin_13_checksum(vec![4, 0, 6, 5, 4, 1, 8, 4, 4, 8, 2, 4]);
        assert_eq!(result, Some(6));
    }
}
