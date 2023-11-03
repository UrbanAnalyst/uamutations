use std::fs::File;
use std::io::BufReader;
use geojson::GeoJson;

pub fn readfile(filename: &str, varname: &str, nentries: usize) -> Vec<f64> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let geojson = GeoJson::from_reader(reader).unwrap();

    let mut values = Vec::new();

    if let GeoJson::FeatureCollection(collection) = geojson {
        for feature in collection.features {
            if values.len() >= nentries {
                break;
            }
            if let Some(properties) = feature.properties {
                if let Some(value) = properties.get(varname) {
                    if let Some(number) = value.as_f64() {
                        values.push(number);
                    }
                }
            }
        }
    }

    values
}
