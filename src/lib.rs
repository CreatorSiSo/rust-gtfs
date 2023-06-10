use serde::de::DeserializeOwned;

pub mod types;

const FILES_TO_LOAD: [&str; 16] = [
	"agency.txt",
	"attributions.txt",
	"calendar.txt",
	"calendar_dates.txt",
	"fare_attributes.txt",
	"fare_rules.txt",
	"feed_info.txt",
	"frequencies.txt",
	"levels.txt",
	"routes.txt",
	"pathways.txt",
	"shapes.txt",
	"stops.txt",
	"stop_times.txt",
	"transfers.txt",
	"trips.txt",
];

const REQUIRED_FILES: [&str; 5] = [
	"agency.txt",
	"routes.txt",
	"stops.txt",
	"stop_times.txt",
	"trips.txt",
];

pub fn parse_file<R: std::io::Read, T: DeserializeOwned>(reader: R) -> csv::Result<Vec<T>> {
	let mut elements = vec![];
	for result in csv::Reader::from_reader(reader).deserialize() {
		elements.push(result?);
	}
	Ok(elements)
}
