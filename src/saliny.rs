use chrono::prelude::*;
use chrono::{Date, DateTime, Utc};

struct TramStation<'a> {
    station_name: String,
    timetables: Vec<TimeTable<'a>>,
    train_access: bool,
}

impl <'a>PartialEq for TramStation<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.station_name == other.station_name
    }
}

struct TramLine<'a> {
    line_number: u16,
    stations: &'a Vec<TramStation<'a>>,
}

struct TimeTable<'a> {
    timetable_station: TramStation<'a>,
    timetable_line: TramLine<'a>,
    timetable_time: Vec<DateTime<Utc>>,
}

///Begin the program from main
pub fn begin_saliny() {
    let mut tram_stations: Vec<TramStation> = Vec::new();
    let mut tram_lines: Vec<TramLine> = Vec::new();

    tram_stations.push(populate_tram_station(
        "Řečkovice".to_string(),
        Vec::new(),
        false,
    ));
    tram_stations.push(populate_tram_station(
        "Filkukova".to_string(),
        Vec::new(),
        false,
    ));

    //adding first tram line to the thingy
    tram_lines.push(populate_tram_line(1, &tram_stations));

    //show the tram stations on a line
    show_stations(&tram_lines[0]);
    println!("{}", is_station_on_line(&tram_lines[0], &tram_stations[0]));
    next_station(&tram_lines[0], &tram_stations[0]);
}

///Show the stations on a line
fn show_stations(line: &TramLine) {
    line.stations
        .iter()
        .for_each(|station| println!("{}", station.station_name))
}

///Return if a station contains a specific one
fn is_station_on_line(line: &TramLine, station: &TramStation) -> bool {
    line.stations.contains(station)
}

///Next stop is
fn next_station(line: &TramLine, station: &TramStation) {
    let index = line.stations.iter().position(|x| x == station).unwrap();
    if index < line.stations.len() - 1 {
        println!(
            "The next station is: {}",
            line.stations[index + 1].station_name
        );
    }
}

///List stations that are in common with each other on a line
fn list_common_stations(line1: &TramLine, line2: &TramLine) {
    line1.stations.iter().for_each(|station| {
        if is_station_on_line(line2, station) {
            println!("{}", station.station_name);
        }
    });
}

///List stations with train station access
fn list_stations_with_train(line1: &TramLine) {
    line1.stations.iter().for_each(|station| {
        if station.train_access == true {
            println!("{}", station.station_name)
        }
    })
}

///Populate the timetable
fn populate_timetable<'a>(
    timetable_station: TramStation<'a>,
    timetable_line: TramLine<'a>,
    timetable_time: Vec<DateTime<Utc>>,
) -> TimeTable<'a> {
    let timetable = TimeTable {
        timetable_station,
        timetable_line,
        timetable_time,
    };
    return timetable;
}

///Populate the tram station
fn populate_tram_station(
    station_name: String,
    timetables: Vec<TimeTable>,
    train_access: bool,
) -> TramStation {
    let tram_station = TramStation {
        station_name,
        timetables,
        train_access,
    };
    return tram_station;
}

///Populate the tram line
fn populate_tram_line<'a>(line_number: u16, stations: &'a Vec<TramStation>) -> TramLine<'a> {
    let tram_line = TramLine {
        line_number,
        stations,
    };
    return tram_line;
}
