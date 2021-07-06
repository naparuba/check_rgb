use chrono::{Utc};
use chrono::{NaiveDate, NaiveDateTime};
use std::io::BufReader;

extern crate ical;
extern crate iso8601;


const DATE_FORMAT_STR: &'static str = "%Y-%m-%d à %H:%M:%S";
const RGB_AGENDA_URI: &'static str = "https://studiorenegade.fr/agenda-5.ics";

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn _get_agenda_as_string() -> String {

    let resp = reqwest::blocking::get(RGB_AGENDA_URI);
    let resp = match resp {
        Ok(resp) => resp,
        Err(error) => panic!("Connection à l'agenda impossible: {:?}", error),
    };

    let buf = resp.text();
    let buf = match buf {
        Ok(buf) => buf,
        Err(error) => panic!("Ouverture de l'agenda impossible: {:?}", error),
    };

    return buf;
}

fn _get_first_date_string_from_agenda(agenda_string: String) -> String {
    let bufreader = BufReader::new(agenda_string.as_bytes());
    let reader = ical::PropertyParser::from_reader(bufreader);


    let mut dtstart = String::from("");
    for line in reader {
        let prop = line.unwrap();
        if prop.name == "DTSTART" {
            dtstart = prop.value.unwrap();
            break;
        }
    }
    return dtstart;
}

fn _get_date_from_datestring(datetime_string: String) -> NaiveDateTime {
    // Now parse the DTSTART as a real date
    let datetime = iso8601::datetime(datetime_string.as_str()).unwrap();

    let native_date: NaiveDateTime;

    // datetime can be several format (3), only one is managed YMD
    match datetime.date {
        iso8601::Date::YMD { year, month, day } => {
            native_date = NaiveDate::from_ymd(year, month, day).and_hms(datetime.time.hour, datetime.time.minute, datetime.time.second);
        }
        _ => {
            println!("<span style='color:red'>ERREUR</span>: le format de date de l'agenda est inconnu");
            std::process::exit(2);
        }
    }
    return native_date;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the agenda, and get the whole string of it
    let agenda_string = _get_agenda_as_string();

    // Parse the agenda until we get a String of DTSTART
    let dtstart = _get_first_date_string_from_agenda(agenda_string);

    // Maybe it's void, no RGB :'(
    if dtstart == "" {
        println!("Pas de RGB planifié actuellement :'( ");
        std::process::exit(0);
    }

    // Parse the datestring, and get a epoch + string representation
    let event_date = _get_date_from_datestring(dtstart);
    let event_epoch = event_date.timestamp();
    let date_str = event_date.format(DATE_FORMAT_STR).to_string();

    // Get the now for comparison
    let now = Utc::now();
    let now_epoch = now.timestamp();

    // Get the number of days from the event
    let diff_float: f64 = (event_epoch - now_epoch) as f64;
    let in_nb_days = (diff_float / 86400.0) as i64;


    // Check it! :)
    if in_nb_days <= 1 {
        println!("<span style='color:red'>ALERTE 🎉</span>: le prochain RGB est quasiment là! Tenez vous prêt! [ <span style='color:purple'>{:?}</span> ]", (date_str));
        std::process::exit(2);
    }


    if in_nb_days <= 3 {
        println!("<span style='color:orange'>ATTENTION ❗</span>: le prochain RGB est très bientôt! (dans {} jours) Réservez votre soirée! [ <span style='color:purple'>{:?}</span> ]", in_nb_days, date_str);
        std::process::exit(1);
    }

    // It's far away
    println!("<span style='color:green'>OK</span>: Le prochain RGB est encore loin (dans {} jours), vous avez le temps [ <span style='color:purple'>{:?}</span> ]", in_nb_days, date_str);
    std::process::exit(0);
}
