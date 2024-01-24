use chrono::NaiveDateTime;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

fn main() {
    gpx_to_csv();
}

fn gpx_to_csv() {
    let files = fs::read_dir("").unwrap();

    for file in files {
        let file_path = file.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        if file_name.ends_with(".gpx") {
            let file = File::open(&file_path).unwrap();
            let reader = BufReader::new(file);
            let mut file =
                File::create(format!("{}.csv", file_name.trim_end_matches(".gpx"))).unwrap();
            writeln!(file, "lon, lat, ele, raw_time, speed, time/s, distance/m").unwrap();
            let mut count = 0;
            let mut caltime0 =
                NaiveDateTime::parse_from_str("2000-01-01T00:00:00Z", "%Y-%m-%dT%H:%M:%SZ")
                    .unwrap();
            let mut ex_lon = 0.0;
            let mut ex_lat = 0.0;
            let mut dis = 0.0;
            let re = Regex::new(r#"<trkpt lon="([^"]+)" lat="([^"]+)"><ele>([^<]+)</ele><time>([^<]+)</time><speed>([^<]+)</speed></trkpt>"#).unwrap();

            for line in reader.lines() {
                let content = line.unwrap();

                for cap in re.captures_iter(&content) {
                    let lon = (&cap[1]).to_string().parse().unwrap();
                    let lat = (&cap[2]).to_string().parse().unwrap();
                    let ele = &cap[3];
                    let raw_time = &cap[4];
                    let speed = &cap[5];
                    let mut time = 0;

                    if count == 0 {
                        caltime0 =
                            NaiveDateTime::parse_from_str(raw_time, "%Y-%m-%dT%H:%M:%SZ").unwrap();
                    } else {
                        let caltime =
                            NaiveDateTime::parse_from_str(raw_time, "%Y-%m-%dT%H:%M:%SZ").unwrap();
                        let duration = caltime.signed_duration_since(caltime0);
                        time = duration.num_seconds();
                        let a = (lat-ex_lat)/180.0*std::f64::consts::PI;
                        let b = (lon-ex_lon)/180.0*std::f64::consts::PI;
                        dis = 1000.0*2.0*6378.137*f64::asin(f64::sqrt(f64::sin(a/2.0).powi(2)+f64::cos(lat)*f64::cos(ex_lat)*f64::sin(b/2.0).powi(2)));
                    }
                    ex_lat = lat;
                    ex_lon = lon;

                    writeln!(
                        file,
                        "{},{},{},{},{},{},{}",
                        lon, lat, ele, raw_time, speed, time, dis
                    )
                    .unwrap();

                    count += 1;
                }
            }
        }
    }
}
