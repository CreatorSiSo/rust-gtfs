use gtfs::{
	parse_file,
	types::{Agency, Route},
};

fn main() -> anyhow::Result<()> {
	let agency_txt = include_str!("../data/20230605_fahrplaene_gesamtdeutschland_gtfs/agency.txt");

	let agencies: Vec<Agency> = parse_file(agency_txt.as_bytes())?;
	println!("{:#?}", agencies[0]);

	let routes_txt = include_str!("../data/20230605_fahrplaene_gesamtdeutschland_gtfs/routes.txt");
	let routes: Vec<Route> = parse_file(routes_txt.as_bytes())?;
	println!(
		"{:#?}",
		routes.iter().find(|route| route.sort_order.is_some())
	);

	Ok(())
}
