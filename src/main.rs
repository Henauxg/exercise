use std::io::{self, BufRead};

fn main() {
    let (valid_count, invalid_count) = match parse_csv_from_stdin() {
        Some((valid_count, invalid_count)) => (valid_count, invalid_count),
        None => (0, 0),
    };

    println!("{valid_count} {invalid_count}");
}

fn parse_csv_from_stdin() -> Option<(usize, usize)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(|l| {
        // Signal reading error by a None value
        let Ok(line) = l else { return Some(None) };
        // Skip empty lines
        if !line.is_empty() {
            Some(Some(line))
        } else {
            None
        }
    });

    // Parse CSV headers
    let headers_line = lines.next()??;
    let ean_column_index = {
        let headers: Vec<&str> = headers_line.split(',').collect();
        // Find the "ean" header
        match headers.iter().position(|c| *c == "ean") {
            Some(column_index) => column_index,
            // "If the header is missing, it will be the first column"
            None => 0,
        }
    };

    let mut valid_count = 0;
    let mut invalid_count = 0;

    // Parse CSV content
    while let Some(line) = lines.next() {
        // Unwrap reading errors
        let line = line?;
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

    Some((valid_count, invalid_count))
}

fn is_valid_gtin_13(ean_str: &str) -> bool {
    // Ignore all leading zeros
    let Some(first_non_zero_index) = ean_str.find(|c| (c != '0')) else {
        return false;
    };
    let ean_str = &ean_str[first_non_zero_index..];

    // Size should now be a GTIN 13
    // TODO: "3" is arbitrary, to reject empty string or ean that are too short. See the spec for more details
    if ean_str.len() > 13 || ean_str.len() < 3 {
        return false;
    }

    let Some(non_padded_ean) = ean_str
        .chars()
        .map(|c| c.to_digit(10))
        .collect::<Option<Vec<u32>>>()
    else {
        return false;
    };

    // TODO Test Prefixes

    // Pad with zeros if < 13
    let zeros_padding_len = 13 - non_padded_ean.len();
    let mut ean = vec![0; zeros_padding_len];
    ean.extend(non_padded_ean);

    // Test checksum
    let theoretical_checksum = gtin_13_checksum(&ean);
    theoretical_checksum == ean[12]
}

/// Assumes ean.len >= 12
fn gtin_13_checksum(ean: &Vec<u32>) -> u32 {
    let sum = ean
        .iter()
        .enumerate()
        .map(|(index, d)| if index % 2 == 0 { *d } else { 3 * d })
        .sum::<u32>();

    let checksum = match sum % 10 {
        0 => 0,
        n => 10 - n,
    };

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksums() {
        let result = gtin_13_checksum(&vec![4, 0, 6, 5, 4, 1, 8, 4, 4, 8, 2, 4]);
        assert_eq!(result, 6);
    }
}
