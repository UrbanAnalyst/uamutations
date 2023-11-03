use std::fs::File;
use std::io::BufReader;
use geojson::GeoJson;

fn main() {
    let file = File::open("dat1.json").unwrap();
    let reader = BufReader::new(file);

    let geojson = GeoJson::from_reader(reader).unwrap();

    let mut values = Vec::new();

    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            if let Some(properties) = feature.properties {
                if let Some(value) = properties.get("transport") {
                    if let Some(number) = value.as_f64() {
                        values.push(number);
                    }
                }
            }
        }
    }

    for number in &values {
        println!("{}", number);
    }
}
