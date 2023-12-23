use anyhow::anyhow;
use std::io::BufReader;
use std::mem;
use std::{error::Error, fs::File};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FitTypeCsv {
    #[serde(rename = "Type Name")]
    pub name: Option<String>,
    #[serde(rename = "Base Type")]
    pub base_type: Option<String>,
    #[serde(rename = "Value Name")]
    pub value_name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

impl FitTypeCsv {
    pub fn new_type(name: &str, base_type: &str) -> FitTypeCsv {
        Self {
            name: Some(name.to_string()),
            base_type: Some(base_type.to_string()),
            value: None,
            value_name: None,
            comment: None,
        }
    }

    pub fn new_value(value_name: &str, value: &str, comment: Option<&str>) -> FitTypeCsv {
        Self {
            name: None,
            base_type: None,
            value: Some(value.to_string()),
            value_name: Some(value_name.to_string()),
            comment: comment.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct FitType {
    pub name: String,
    pub base_type: String,
    pub values: Vec<FitTypeValue>,
}

#[derive(Debug)]
pub struct FitTypeValue {
    pub name: String,
    pub value: String,
    pub comment: String,
}

pub fn read_types(path: &str) -> Result<Vec<FitTypeCsv>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);

    let mut records = vec![];

    for result in csv_reader.deserialize() {
        let record: FitTypeCsv = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn convert_types(csv_types: &Vec<FitTypeCsv>) -> Result<Vec<FitType>, anyhow::Error> {
    if csv_types.is_empty() {
        Err(anyhow!("csv_types is empty"))?
    }

    let mut fit_types = vec![];
    let mut curr_fit_type: Option<&FitTypeCsv> = None;
    let mut curr_fit_type_values = vec![];

    let mut it = csv_types.iter().peekable();
    while let Some(csv_type) = it.next() {
        if csv_type.name.is_some() {
            curr_fit_type = Some(csv_type);
        } else {
            let value = FitTypeValue {
                name: csv_type
                    .value_name
                    .as_ref()
                    .ok_or(anyhow!("missing value name"))?
                    .to_owned(),
                value: csv_type
                    .value
                    .as_ref()
                    .ok_or(anyhow!("missing value"))?
                    .trim()
                    .to_owned(),
                comment: csv_type.comment.as_ref().unwrap_or(&"".to_string()).to_owned(),
            };

            curr_fit_type_values.push(value);
        }

        if it.peek().is_some_and(|x| x.name.is_some()) || it.peek().is_none() {
            if let Some(cft) = curr_fit_type {
                let fit_type = FitType {
                    name: cft.name.as_ref().unwrap().to_owned(),
                    base_type: cft.base_type.as_ref().ok_or(anyhow!("missing base type"))?.to_owned(),
                    values: mem::take(&mut curr_fit_type_values), //curr_fit_type_values.drain(..).collect()
                };

                fit_types.push(fit_type);
            }
        }
    }

    Ok(fit_types)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_types_works() {
        let values: Vec<FitTypeCsv> = vec![
            FitTypeCsv::new_type("type_name_1", "base_type_1"),
            FitTypeCsv::new_value("value_name_1_1", "1", Some("comment_1_1")),
            FitTypeCsv::new_value("value_name_1_2", "2", None),
            FitTypeCsv::new_value("value_name_1_3", "3", None),
            FitTypeCsv::new_type("type_name_2", "base_type_2"),
            FitTypeCsv::new_value("value_name_2_1", "4", Some("comment_2_1")),
            FitTypeCsv::new_value("value_name_2_2", "5", None),
            FitTypeCsv::new_value("value_name_2_3", "6", None),
        ];

        let result = convert_types(&values);
        assert!(result.is_ok());

        let types = result.unwrap();
        assert_eq!(types.len(), 2);

        let type1 = &types[0];
        assert_eq!(type1.name, "type_name_1");
        assert_eq!(type1.base_type, "base_type_1");
        assert_eq!(type1.values[0].name, "value_name_1_1");
        assert_eq!(type1.values[0].value, "1");
        assert_eq!(type1.values[0].comment, "comment_1_1");
        assert_eq!(type1.values[1].name, "value_name_1_2");
        assert_eq!(type1.values[1].value, "2");
        assert_eq!(type1.values[1].comment, "");
        assert_eq!(type1.values[2].name, "value_name_1_3");
        assert_eq!(type1.values[2].value, "3");
        assert_eq!(type1.values[2].comment, "");

        let type2 = &types[1];
        assert_eq!(type2.name, "type_name_2");
        assert_eq!(type2.base_type, "base_type_2");
        assert_eq!(type2.values[0].name, "value_name_2_1");
        assert_eq!(type2.values[0].value, "4");
        assert_eq!(type2.values[0].comment, "comment_2_1");
        assert_eq!(type2.values[1].name, "value_name_2_2");
        assert_eq!(type2.values[1].value, "5");
        assert_eq!(type2.values[1].comment, "");
        assert_eq!(type2.values[2].name, "value_name_2_3");
        assert_eq!(type2.values[2].value, "6");
        assert_eq!(type2.values[2].comment, "");
    }
}
