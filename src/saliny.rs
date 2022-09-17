use chrono::prelude::*;
use chrono::{Date, DateTime, Utc};

struct TramStation {
    station_name: String,
    timetables: Vec<TimeTable>,
    train_access: bool,
}

impl PartialEq for TramStation {
    fn eq(&self, other: &Self) -> bool {
        self.station_name == other.station_name
    }
}

struct TramLine {
    line_number: u16,
    line_name: String,
    stations: Vec<TramStation>,
}

struct TimeTable {
    timetable_station: TramStation,
    timetable_line: TramLine,
    timetable_time: Vec<DateTime<Utc>>,
}

pub fn begin_saliny() {
    let mut tram_stations: Vec<TramStation> = Vec::new();
    let mut tram_line: Vec<TramLine> = Vec::new();
}

fn show_stations(line: &TramLine) {
    line.stations
        .iter()
        .for_each(|station| println!("{}", station.station_name))
}

fn is_station_on_line(line: &TramLine, station: &TramStation) -> bool {
    line.stations.contains(station)
}

fn next_station(line: &TramLine, station: &TramStation) {
    let index = line.stations.iter().position(|x| x == station).unwrap();
    if index < line.stations.len()-1 {
        println!("The next station is: {}", line.stations[index + 1].station_name);
    }
}

fn list_common_stations(line1: &TramLine, line2: &TramLine) {
    line1.stations.iter().for_each(|station| {
        if is_station_on_line(line2, station) {
            println!("{}", station.station_name);
        }
    });
}

fn list_stations_with_train(line1: &TramLine) {
    line1.stations.iter().for_each(|station| {
        if station.train_access == true {
            println!("{}", station.station_name)
        }
    })
}

