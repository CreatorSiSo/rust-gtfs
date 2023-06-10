use crate::types::Color;
use serde_repr::Deserialize_repr;

#[derive(Debug, serde::Deserialize)]
/// https://gtfs.org/schedule/reference/#routestxt
pub struct Route {
	#[serde(rename = "route_id")]
	pub id: String,
	#[serde(rename = "agency_id")]
	pub agency_id: String,
	#[serde(rename = "route_short_name")]
	/// Short name of a route. Often a short, abstract identifier
	/// (e.g., "32", "100X", "Green") that riders use to identify a route.
	/// Both [`Route::short_name`] an [`Route::long_name`] may be defined.
	///
	/// Conditionally Required:
	/// - Required if [`Route::long_name`] is empty.
	/// - Optional otherwise.
	pub short_name: String,
	#[serde(rename = "route_long_name")]
	/// Full name of a route. This name is generally more descriptive than
	/// the short_name and often includes the route's destination or stop.
	/// Both [`Route::short_name`] an [`Route::long_name`] may be defined.
	///
	///  Conditionally Required:
	/// - Required if [`Route::short_name`] is empty.
	/// - Optional otherwise.
	pub long_name: String,
	#[serde(rename = "route_desc")]
	pub desc: Option<String>,
	#[serde(rename = "route_type")]
	pub kind: RouteKind,
	#[serde(rename = "route_url")]
	pub url: Option<String>,
	#[serde(rename = "route_color")]
	pub color: Option<Color>,
	#[serde(rename = "route_text_color")]
	pub text_color: Option<Color>,
	#[serde(rename = "route_sort_order")]
	pub sort_order: Option<usize>,
	#[serde(rename = "continuous_pickup")]
	pub continuous_pickup: Option<ContinuousStopping>,
	#[serde(rename = "continuous_drop_off")]
	pub continuous_drop_off: Option<ContinuousStopping>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u16)]
/// Indicates the type of transportation used on a route.
///
/// - https://developers.google.com/transit/gtfs/reference#routestxt
/// - https://developers.google.com/transit/gtfs/reference/extended-route-types
pub enum RouteKind {
	/// Tram, Streetcar, Light rail.
	/// Any light rail or street level system within a metropolitan area.
	Tram = 0,
	/// Subway, Metro.
	/// Any underground rail system within a metropolitan area.
	Metro = 1,
	/// Rail.
	/// Used for intercity or long-distance travel.
	Rail = 2,
	/// Bus.
	/// Used for short- and long-distance bus routes.
	Bus = 3,
	/// Ferry.
	/// Used for short- and long-distance boat service.
	Ferry = 4,
	/// Cable tram.
	/// Used for street-level rail cars where the cable runs beneath the vehicle,
	/// e.g., cable car in San Francisco.
	CableTram = 5,
	/// Aerial lift, suspended cable car (e.g., gondola lift, aerial tramway).
	/// Cable transport where cabins, cars,
	/// gondolas or open chairs are suspended by means of one or more cables.
	AerialLift = 6,
	/// Funicular.
	/// Any rail system designed for steep inclines.
	Funicular = 7,
	/// Trolleybus.
	/// Electric buses that draw power from overhead wires using poles.
	Trolleybus = 11,
	/// Monorail.
	/// Railway in which the track consists of a single rail or a beam.
	Monorail = 12,

	// Extended Route Types
	// Use Hierarchical Vehicle Type (HVT) codes from the European TPEG standard.
	// https://developers.google.com/transit/gtfs/reference/extended-route-types
	//
	RailwayServicetApplicable = 100,
	/// TGV (FR), ICE (DE), Eurostar (GB)
	HighSpeedRailService = 101,
	/// InterCity/EuroCity
	LongDistanceTrains = 102,
	/// InterRegio (DE), Cross County Rail (GB)
	InterRegionalRailService = 103,
	CarTransportRailService = 104,
	/// GNER Sleeper (GB)
	SleeperRailService = 105,
	/// TER (FR), Regionalzug (DE)
	RegionalRailService = 106,
	/// Romney, Hythe & Dymchurch (GB)
	TouristRailwayService = 107,
	/// Rail Shuttle (Within Complex)
	/// Gatwick Shuttle (GB), Sky Line (DE)
	RailShuttle = 108,
	/// S-Bahn (DE), RER (FR), S-tog (Kopenhagen)
	SuburbanRailway = 109,
	ReplacementRailService = 110,
	SpecialRailService = 111,
	LorryTransportRailService = 112,
	AllRailService = 113,
	CrossCountryRailService = 114,
	VehicleTransportRailService = 115,
	/// Rochers de Naye (CH), Dolderbahn (CH)
	RackAndPinionRailway = 116,
	AdditionalRailService = 117,
	CoachService = 200,
	/// EuroLine, Touring
	InternationalCoachService = 201,
	/// National Express (GB)
	NationalCoachService = 202,
	/// Roissy Bus (FR), Reading-Heathrow (GB)
	ShuttleCoachService = 203,
	RegionalCoachService = 204,
	SpecialCoachService = 205,
	SightseeingCoachService = 206,
	TouristCoachService = 207,
	CommuterCoachService = 208,
	AllCoachService = 209,
	UrbanRailwayService = 400,
	/// Métro de Paris
	MetroService = 401,
	/// London Underground, U-Bahn
	UndergroundService = 402,
	UrbanRailwayService2 = 403,
	AllUrbanRailwayService = 404,
	Monorai = 405,
	BusService = 700,
	/// Eastbourne-Maidstone (GB)
	RegionalBusService = 701,
	/// X19 Wokingham-Heathrow (GB)
	ExpressBusService = 702,
	/// 38 London: Clapton Pond-Victoria (GB)
	StoppingBusService = 703,
	LocalBusService = 704,
	/// N prefixed buses in London (GB)
	NightBusService = 705,
	/// Maidstone P4 (GB)
	PostBusService = 706,
	SpecialNeedsBus = 707,
	MobilityBusService = 708,
	MobilityBusRegisteredDisabled = 709,
	SightseeingBus = 710,
	/// 747 Heathrow-Gatwick Airport Service (GB)
	ShuttleBus = 711,
	SchoolBus = 712,
	SchoolandPublicServiceBus = 713,
	RailReplacementBusService = 714,
	DemandandResponseBusService = 715,
	AllBusService = 716,
	TrolleybusService = 800,
	TramService = 900,
	CityTramService = 901,
	/// Munich (DE), Brussels (BE), Croydon (GB)
	LocalTramService = 902,
	RegionalTramService = 903,
	/// Blackpoo lSeafront (GB)
	SightseeingTramService = 904,
	ShuttleTramService = 905,
	AllTramService = 906,
	WaterTransportService = 1000,
	AirService = 1100,
	FerryService = 1200,
	/// Telefèric de Montjuïc (ES), Saleve (CH), Roosevelt Island Tramway (US)
	AerialLiftService = 1300,
	TelecabinService = 1301,
	CableCarService = 1302,
	ElevatorService = 1303,
	ChairLiftService = 1304,
	DragLiftService = 1305,
	SmallTelecabinService = 1306,
	AllTelecabinService = 1307,
	/// Rigiblick (Zürich, CH)
	FunicularService = 1400,
	TaxiService = 1500,
	/// Marshrutka (RU), dolmuş (TR)
	CommunalTaxiService = 1501,
	WaterTaxiService = 1502,
	RailTaxiService = 1503,
	BikeTaxiService = 1504,
	LicensedTaxiService = 1505,
	PrivateHireServiceVehicle = 1506,
	AllTaxiService = 1507,
	MiscellaneousService = 1700,
	HorsedrawnCarriage = 1702,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
/// Indicates whether a rider can board/leave the transit vehicle
/// anywhere along the vehicle’s travel path.
/// The path is described by shapes.txt on every trip of the route.
///
/// The default continuous pickup behavior defined in routes.txt
/// can be overridden in stop_times.txt.
pub enum ContinuousStopping {
	/// Continuous stopping pickup/drop-off.
	Yes = 0,
	/// No continuous stopping pickup/drop-off.
	No = 1,
	/// Must phone an agency to arrange continuous stopping pickup/drop-off.
	Phone = 2,
	/// Must coordinate with a driver to arrange continuous stopping pickup/drop-off.
	Driver = 3,
}
