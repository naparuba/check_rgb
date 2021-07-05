//use std::collections::HashMap;
extern crate ical;
extern crate iso8601;
use chrono::{Datelike, Timelike, Utc};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

// import ical lib
use std::io::BufReader;

const DATE_FORMAT_STR: &'static str = "%Y-%m-%d à %H:%M:%S";

fn print_type_of<T>(_: &T) {
    //println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("Hello, world!");
    let uri = "https://studiorenegade.fr/agenda.ics";

    let resp = reqwest::blocking::get(uri)?;
    //.json::<HashMap<String, String>>()?;
    //println!("{:#?}", resp);
    let buf = resp.text()?;
    //println!("{:#?}", buf);

    let bufreader = BufReader::new(buf.as_bytes());
    let reader = ical::PropertyParser::from_reader(bufreader);


    let mut dtstart = String::from("");
    for line in reader {
        //println!("{:?}", line);
        let prop = line.unwrap();

        //println!("NAME: {:?}", prop.name);
        if (prop.name == "DTSTART") {
            //println!("START: {:?}", prop.value);
            dtstart = prop.value.unwrap();
        }
    }

    if (dtstart == "") {
        //println!("No RGB planned");
        return Ok(());
    }

    //println!("DTSTART: {:?}", dtstart.as_str());
    //let rfc3339 = DateTime::parse_from_rfc3339(dtstart.as_str())?;
    //println!("{:?} : {:?}", dtstart.as_str(), rfc3339);
    //let datetime = iso8601::datetime("2015-06-26T16:43:23+0200").unwrap();
    let datetime = iso8601::datetime(dtstart.as_str()).unwrap();
    //println!("DATE= {:?}", datetime.date);

    let native_date : NaiveDateTime;
    let mut event_epoch = 0;
    match datetime.date {
        iso8601::Date::YMD { year,     month, day } => {
            //println!("YMD format value: {} {} {}", year, month, day);
            native_date = NaiveDate::from_ymd(year, month, day).and_hms(datetime.time.hour, datetime.time.minute, datetime.time.second);
            event_epoch = native_date.timestamp();
            //println!("Datetime: {}", native_date);
        },
        _ => {println!("Something else");
            return Ok(());
        },
    }
    //println!("TIME= {:?}", datetime.time);


    let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
    //println!(
    //    "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
    //    date_time, date_time.timestamp());

    let now = Utc::now();
    let now_epoch = now.timestamp();


    print_type_of(&datetime);
    //println!("Event in {} ({} - {})", event_epoch - now_epoch, now_epoch, event_epoch);
    let mut diff_float: f64 = (event_epoch - now_epoch) as f64;
    let in_nb_days = (diff_float / 86400.0) as i64;
    //println!("Event in {} days", in_nb_days);

    let date_str = native_date.format(DATE_FORMAT_STR).to_string();

    if in_nb_days <= 1 {
        println!("<span style='color:red'>ALERTE 🎉</span>: le prochain RGB est quasiment là! Tenez vous prêt! [ <span style='color:purple'>{:?}</span> ]", (date_str));
        return Ok(());
    }


    if in_nb_days <= 3 {
        println!("<span style='color:orange'>ATTENTION ❗</span>: le prochain RGB est très bientôt! (dans {} jours) Réservez votre soirée! [ <span style='color:purple'>{:?}</span> ]", in_nb_days, date_str);
        return Ok(());
    }

    if in_nb_days >= 7 {
        println!("<span style='color:green'>OK</span>: Le prochain RGB est encore loin (dans {} jours), vous avez le temps [ <span style='color:purple'>{:?}</span> ]", in_nb_days, date_str);
        return Ok(());
    }

    return Ok(());
}
