use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};
use csv::{Writer, WriterBuilder};
use serde_json::{Map, Value};

const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESC: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn process_input<R: BufRead, W: std::io::Write>(
    reader: &mut R,
    writer: &mut Writer<W>,
) -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let obj: Map<String, Value> = serde_json::from_str(&line)?;
        let mut record: Vec<String> = Vec::new();
        for key in obj.keys() {
            match &obj[key] {
                Value::String(s) => record.push(s.to_string()),
                other => {
                    let stringified = format!("{}", other);
                    record.push(stringified);
                }
            }
        }
        writer.write_record(record)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("j2c")
        .version(VERSION)
        .author(AUTHORS)
        .about(DESC)
        .arg(
            Arg::new("header")
                .about("CSV column header")
                .short('H')
                .long("header")
                .value_name("HEADER")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::new("input")
                .about("One or more newline-delimited JSON files")
                .value_name("INPUT_FILE")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    println!("{:#?}", matches);

    let mut output = WriterBuilder::new().from_writer(std::io::stdout());

    if let Some(headers) = matches.values_of("header") {
        output.write_record(headers)?;
    }

    if let Some(inputs) = matches.values_of("input") {
        for input in inputs {
            let mut reader = BufReader::new(File::open(input).unwrap());
            process_input(&mut reader, &mut output)?;
        }
    } else {
        let mut reader = BufReader::new(io::stdin());
        process_input(&mut reader, &mut output)?;
    }
    output.flush()?;
    Ok(())
}
