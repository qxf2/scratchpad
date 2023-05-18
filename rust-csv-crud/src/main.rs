use csv::ReaderBuilder;
use std::fs::File;
use std::io::ErrorKind;

fn open_file(csv_path: &str) -> Result<File, Box<dyn std::error::Error>> {

    match File::open(csv_path) {
        Ok(file) => Ok(file),
        Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => {
                        println!("File doesn't exist");
                    }
                    ErrorKind::AddrInUse => {
                        println!("A socket address could not be bound because the address is already in use elsewhere.");
                    }
                    ErrorKind::PermissionDenied => {
                        println!("The operation lacked the necessary privileges to complete.");
                    }
                    ErrorKind::ConnectionRefused => {
                        println!("The connection was refused by the remote server.");
                    }
                    ErrorKind::ConnectionReset => {
                        println!("The connection was reset by the remote server.");
                    }
                    ErrorKind::ConnectionAborted => {
                        println!("The connection was aborted (terminated) by the remote server.");
                    }
                    ErrorKind::NotConnected => {
                        println!("The network operation failed because it was not connected yet.");
                    }
                    ErrorKind::AddrNotAvailable => {
                        println!("A nonexistent interface was requested or the requested address was not local.");
                    }
                    ErrorKind::BrokenPipe => {
                        println!("The operation failed because a pipe was closed.");
                    }
                    ErrorKind::AlreadyExists => {
                        println!("An entity already exists, often a file.");
                    }
                    ErrorKind::WouldBlock => {
                        println!("The operation needs to block to complete, but the blocking operation was requested to not occur.");
                    }
                    ErrorKind::InvalidInput => {
                        println!("Data not valid for the operation were encountered.");
                    }
                    ErrorKind::TimedOut => {
                        println!("The I/O operation’s timeout expired, causing it to be canceled.");
                    }
                    ErrorKind::WriteZero => {
                        println!("An error returned when an operation could not be completed because a call to write returned Ok(0).");
                    }
                    ErrorKind::Interrupted => {
                        println!("This operation was interrupted.");
                    }
                    ErrorKind::Unsupported => {
                        println!("This operation is unsupported on this platform.");
                    }
                    ErrorKind::UnexpectedEof => {
                        println!("An error returned when an operation could not be completed because an “end of file” was reached prematurely.");
                    }
                    ErrorKind::OutOfMemory => {
                        println!("An operation could not be completed, because it failed to allocate enough memory.");
                    }
                    ErrorKind::Other => {
                        println!("There might be something else which is going wrong");
                    }
                    _ => {
                        println!("An error occurred: {:?}", err);
                    }
                }
                return Err(Box::new(err));
        }
    }
}

fn read_csv_details(
    csv_path: &str,
    flag_ignore_error: bool,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let file = match open_file(csv_path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut csv_data: Vec<Vec<String>> = Vec::new();
    let mut error_occurred = false;

    for record in reader.into_records() {
        let record = match record {
            Ok(record) => record,
            Err(err) => {
                if flag_ignore_error {
                    error_occurred = true;
                    continue;
                } else {
                    return Err(Box::new(err));
                }
            }
        };

        let row: Vec<String> = record
            .iter()
            .map(|field| field.trim().to_string())
            .collect();
                    
        csv_data.push(row);        
    }  

    if flag_ignore_error && error_occurred {    
        Ok(csv_data)
    } else {
        Ok(csv_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

#[test]
fn test_read_empty_csv_file() {
    let csv_path = "empty.csv";
    let result = read_csv_details(csv_path, true);
    assert!(result.is_ok(), "Failed to read CSV file");

    let csv_data = result.unwrap();
    assert!(csv_data.is_empty(), "CSV data is expected to be empty");
}

#[test]
fn test_csv_row_count() {
    let csv_path = "cyborg.csv";
    let result = read_csv_details(csv_path, true);
    assert!(result.is_ok(), "Failed to read CSV file");

    let expected_row_count = 2;
    let actual_data = result.unwrap();
    assert_eq!(
        actual_data.len(),
        expected_row_count,
        "Unexpected number of rows in CSV"
    );
}

#[test]
fn test_csv_row_null_values() {
    let csv_path = "cyborg.csv";
    let result = read_csv_details(csv_path, false);
    assert!(result.is_ok(), "Failed to read CSV file");

    let csv_data = dbg!(result.unwrap());

    for (row_count, row) in csv_data.iter().enumerate() {
        assert!(
            !row.iter().any(|value| value.is_empty()),
            "Null value found in row {}",
            row_count + 1
        );
    }
}

#[test]
fn test_csv_column_count() {
    let csv_path = "cyborg.csv";
    let result = read_csv_details(csv_path, true);
    assert!(result.is_ok(), "Failed to read CSV file");

    let csv_data = result.unwrap();
    let expected_column_count = 3;

    for (row_count, row) in csv_data.iter().enumerate() {
        assert_eq!(
            row.len(),
            expected_column_count,
            "Unexpected number of columns in row {}",
            row_count + 1
        );
    }
}

#[test]
fn test_invalid_csv_file() {
    let csv_path = "invalid.csv";
    let result = read_csv_details(csv_path, true);
    assert!(result.is_err(), "Expected an error while reading CSV file");
}

#[test]
fn test_csv_data_contains_special_characters() {
    let csv_path = "cyborg.csv";
    let result = read_csv_details(csv_path, true).unwrap();

    let special_characters = "!@$%^&*()_+=[]{}|<>?";

    for (row_index, row) in result.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            assert!(
                !cell.chars().any(|c| special_characters.contains(c)),
                "Special character found in CSV data at row {}, column {}",
                row_index + 1,
                column_index + 1
            );
        }
    }
}

#[test]
fn test_read_large_csv_file() {
    let csv_path = "large.csv";
    let result = read_csv_details(csv_path, true);
    assert!(result.is_ok(), "Failed to read CSV file");

    let csv_data = result.unwrap();
    assert!(!csv_data.is_empty(), "CSV data should not be empty");

    let expected_row_count = 10000;
    let expected_column_count = 5;

    assert_eq!(
        csv_data.len(),
        expected_row_count,
        "Unexpected number of rows in CSV"
    );

    for (row_count, row) in csv_data.iter().enumerate() {
        assert_eq!(
            row.len(),
            expected_column_count,
            "Unexpected number of columns in row {}",
            row_count + 1
        );
    }
}

fn main() {
    println!("The program performs open and read operation")
}
