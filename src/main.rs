use std::fs::File;
use std::io::BufReader;
use geojson::GeoJson;

const NENTRIES: usize = 100;
const FILENAME: &str = "dat1.json";
const VARNAME: &str = "transport";

fn main() {

    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let geojson = GeoJson::from_reader(reader).unwrap();

    let mut values = Vec::new();

    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            if values.len() >= NENTRIES {
                break;
            }
            if let Some(properties) = feature.properties {
                if let Some(value) = properties.get(VARNAME) {
                    if let Some(number) = value.as_f64() {
                        values.push(number);
                    }
                }
            }
        }
    }

    for (index, number) in values.iter().enumerate() {
        println!("{}: {}", index + 1, number);
    }
}
