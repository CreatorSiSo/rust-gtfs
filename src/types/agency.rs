use chrono_tz::Tz;
use language_tags::LanguageTag;

#[derive(Debug, serde::Deserialize)]
/// https://gtfs.org/schedule/reference/#routestxt
pub struct Agency {
	#[serde(rename = "agency_id")]
	pub id: String,
	#[serde(rename = "agency_name")]
	pub name: String,
	#[serde(rename = "agency_url")]
	pub url: String,
	#[serde(rename = "agency_timezone")]
	pub timezone: Tz,
	#[serde(rename = "agency_lang")]
	pub lang: Option<LanguageTag>,
	#[serde(rename = "agency_phone")]
	pub phone: Option<String>,
	#[serde(rename = "agency_fare_url")]
	pub fare_url: Option<String>,
	#[serde(rename = "agency_email")]
	pub email: Option<String>,
}
