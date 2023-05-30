use csv::{ReaderBuilder,Writer};
use chrono::NaiveDate;
use std::error::Error;

struct Cyborg {
    name: String,
    model: String,
    organization: String,
    abilities: String,
    creation_date: Option<NaiveDate>
}

fn get_cyborgs() -> Vec<Cyborg> {
    vec![
        Cyborg {
            name: "T-800".to_string(),
            model: "4TU".to_string(),
            organization: "Skynet".to_string(),
            abilities: "facial identification".to_string(),
            creation_date: NaiveDate::from_ymd_opt(2015, 12, 7),
        },
        Cyborg {
            name: "C-3PO".to_string(),
            model: "3PO".to_string(),
            organization: "Skywalker".to_string(),
            abilities: "etiquette".to_string(),
            creation_date: NaiveDate::from_ymd_opt(2015, 12, 12),
        },
        Cyborg {
            name: "U-2IO".to_string(),
            model: "2IO".to_string(),
            organization: "Skynet".to_string(),
            abilities: "manner".to_string(),
            creation_date: NaiveDate::from_ymd_opt(2015, 4, 3),
        },        

    ]
}

fn write_csv_details(csv_path: &str) -> Result<(), Box<dyn Error>> {

    let cyborgs = get_cyborgs();

    let writer_result = Writer::from_path(csv_path);
    let mut writer = match writer_result {
        Ok(writer) => writer,
        Err(err) => return Err(Box::new(err)),
    };

    writer.write_record(&["name", "model", "organization", "abilities", "creation_date"]);

    for cyborg in cyborgs {
       let record_result = match writer.write_record(&[
            cyborg.name,
            cyborg.model,
            cyborg.organization,
            cyborg.abilities,
            cyborg.creation_date.map(|date| date.to_string()).unwrap_or_else(|| "1500, 4, 5".to_string()),
        ]) {
            Ok(record_result) => record_result,
            Err(err) => return Err(Box::new(err)),            
        };
    }

    Ok(())

}

fn read_csv_details(
    csv_path: &str,
    flag_ignore_error: bool,
    is_header_present: bool,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {

    let reader_result = ReaderBuilder::new().has_headers(is_header_present).from_path(csv_path);
    let reader = match reader_result {
        Ok(reader) => reader,
        Err(err) => return Err(Box::new(err)),
    };    

    let mut csv_data: Vec<Vec<String>> = Vec::new();

    for record in reader.into_records() {
        let record = match record {
            Ok(record) => record,
            Err(err) => {
                if flag_ignore_error {
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

    Ok(csv_data)
}

#[cfg(test)]
mod tests {
    use super::*;
}

#[test]
fn test_read_empty_csv_file() {
    let csv_path = "empty.csv";
    let result = read_csv_details(csv_path, true, true);
    assert!(result.is_ok(), "Failed to read CSV file");

    let csv_data = result.unwrap();
    assert!(csv_data.is_empty(), "CSV data is expected to be empty, but found records in {} rows", csv_data.len());
}

#[test]
fn test_csv_column_count() {
    let csv_path = "cyborg.csv";
    let result = read_csv_details(csv_path, true,true);
    assert!(result.is_ok(), "Failed to read CSV file");

    let csv_data = result.unwrap();
    let expected_column_count = 3;

    for (row_count, row) in csv_data.iter().enumerate() {
        assert_eq!(
            row.len(),
            expected_column_count,
            "Unexpected number of columns in row {}.Expected: {}, Actual: {}",
            row_count + 1,
            expected_column_count,
            row.len()
        );
    }
}

#[test]
fn test_invalid_csv_file() {
    let csv_path = "invalid.csv";
    let result = read_csv_details(csv_path, false,true);
    assert!(result.is_ok(), "An error occured while reading the CSV file");
}

#[test]
fn test_csv_data_contains_special_characters() {
    let csv_path = "cyborg.csv";
    let result = read_csv_details(csv_path, true,true).unwrap();

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
    let result = read_csv_details(csv_path, true, true);

    let csv_data = result.unwrap();
    let expected_row_count = 10000;
    let expected_column_count = 15;

    assert_eq!(
        csv_data.len(),
        expected_row_count,
        "Unexpected number of rows in CSV. Expected: {}, Actual: {}",
        expected_row_count,
        csv_data.len()
    );

    for (row_count, row) in csv_data.iter().enumerate() {
        assert_eq!(
            row.len(),
            expected_column_count,
            "Unexpected number of columns in row {}. Expected: {}, Actual: {}",
            row_count + 1,
            expected_column_count,
            row.len()
        );
    }
}


fn main() {
    let result = write_csv_details("write_file.csv");
}
