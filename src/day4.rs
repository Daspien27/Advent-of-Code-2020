use regex::Regex;

#[derive(Debug)]
struct Year {
    value: u64
}

#[derive(Clone, Copy, Debug)]
enum Height {
    Cm(u64),
    In(u64),
}

#[derive(Debug)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other
}

#[derive(Debug)]
pub struct ValidatedPassport {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Height>,
    hair_color:Option<String>,
    eye_color: Option<EyeColor>,
    passport_id: Option<String>,
    country_id: Option<String>,
}


impl ValidatedPassport {
    fn valid(&self) -> bool {
        !(self.birth_year.is_none()
        || self.issue_year.is_none()
        || self.expiration_year.is_none()
        || self.height.is_none()
        || self.hair_color.is_none()
        || self.eye_color.is_none()
        || self.passport_id.is_none())
    }
}

fn validate_year(year_str: &Option<String>, lower: u64, upper: u64) -> Option<Year> {
    year_str
        .as_ref()
        .filter(|s| {
            lazy_static! {
                static ref YEAR_RE : Regex = Regex::new(r"\d{4}").unwrap();
            }

            YEAR_RE.is_match(s.as_str())
        })
        .map(|s| {
            Year{value: s.parse().unwrap()}
        })
        .filter(|y| {
            y.value >= lower && y.value <= upper
        })
}

fn validate_birth_year(year_str : &Option<String>) -> Option<Year> {
    validate_year(&year_str, 1920, 2002)
}

fn validate_issue_year(year_str : &Option<String>) -> Option<Year> {
    validate_year(&year_str, 2010, 2020)
}

fn validate_expiration_year (year_str : &Option<String>) -> Option<Year> {
    validate_year(&year_str, 2020, 2030)
}

fn validate_height(height_str : &Option<String>) -> Option<Height> {
    height_str
        .as_ref()
        .and_then(|s| {
            lazy_static! {
                static ref HEIGHT_RE : Regex = Regex::new(r"(?P<measure>\d+)(?P<unit>cm|in)").unwrap();
            }

            HEIGHT_RE
                .captures(s)
                .and_then(|caps|{
                match caps.name("unit")?.as_str() {
                    "cm" => Some(Height::Cm(caps.name("measure")?.as_str().parse::<u64>().unwrap())),
                    "in" => Some(Height::In(caps.name("measure")?.as_str().parse::<u64>().unwrap())),
                    _ => None,
                }
            })
        })
        .filter(|h| {
            match h {
                Height::Cm(val) => (150 <= *val && *val <= 193),
                Height::In(val) => (59 <= *val && *val <= 76)
            }
        })
}

fn validate_eye_color(eye_color_str: &Option<String>) -> Option<EyeColor> {
    eye_color_str.as_ref().and_then(|s| {
        match s.as_str() {
            "amb" => Some(EyeColor::Amber),
            "blu" => Some(EyeColor::Blue),
            "brn" => Some(EyeColor::Brown),
            "gry" => Some(EyeColor::Gray),
            "grn" => Some(EyeColor::Green),
            "hzl" => Some(EyeColor::Hazel),
            "oth" => Some(EyeColor::Other),
            _ => None

        }
    })    
}

fn validate_hair_color(hair_color_str: &Option<String>) -> Option<String> {
    hair_color_str
        .clone()
        .filter(|hc| {
            lazy_static! {
                static ref HAIR_RE : Regex = Regex::new(r"#[[:xdigit:]]{6}").unwrap();
            }
            HAIR_RE.is_match(hc.as_str())
        })
}

fn validate_passport_id(passport_id_str: &Option<String>) -> Option<String> {
    passport_id_str
        .clone()
        .filter(|pid|{
            lazy_static! {
                static ref PID_RE : Regex = Regex::new(r"^\d{9}?$").unwrap();
            }
            PID_RE.is_match(pid.as_str())
        })
}

impl From<&Passport> for ValidatedPassport {
    fn from(passport: &Passport) -> Self {
        ValidatedPassport {birth_year:      validate_birth_year(&passport.birth_year),
                           issue_year:      validate_issue_year(&passport.issue_year),
                           expiration_year: validate_expiration_year(&passport.expiration_year),
                           height:          validate_height(&passport.height),
                           hair_color:      validate_hair_color(&passport.hair_color),
                           eye_color:       validate_eye_color(&passport.eye_color),
                           passport_id:     validate_passport_id(&passport.passport_id),
                           country_id: passport.country_id.clone()}
    }
}

#[derive(Debug)]
pub struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color:Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn empty_passport() -> Self {
        Passport{birth_year: None,
                 issue_year:  None,
                 expiration_year: None,
                 height: None,
                 hair_color: None,
                 eye_color: None,
                 passport_id: None,
                 country_id: None}
    }

    fn valid(&self) -> bool {
        self.birth_year.is_some()
        && self.issue_year.is_some()
        && self.expiration_year.is_some()
        && self.height.is_some()
        && self.hair_color.is_some()
        && self.eye_color.is_some()
        && self.passport_id.is_some()
    }
}

#[derive(Debug)]
pub struct Feature {
    code : String,
    value: String,
}

impl From<&Vec<Feature>> for Passport {
     fn from(features: &Vec<Feature>) -> Self {
         let mut passport = Passport::empty_passport();

         features
            .iter()
            .for_each(|f|
            {
                match f.code.as_str()
                {
                    "byr" => {passport.birth_year      = Some(f.value.clone()); }
                    "iyr" => {passport.issue_year      = Some(f.value.clone()); }
                    "eyr" => {passport.expiration_year = Some(f.value.clone()); }
                    "hgt" => {passport.height          = Some(f.value.clone()); }
                    "hcl" => {passport.hair_color      = Some(f.value.clone()); }
                    "ecl" => {passport.eye_color       = Some(f.value.clone()); }
                    "pid" => {passport.passport_id     = Some(f.value.clone()); }
                    "cid" => {passport.country_id      = Some(f.value.clone()); }
                    _ => { panic!("Unexpected feature!"); }
                }
            });

        passport
     }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"(?m)(?:^.+$\n?)+").unwrap();
        static ref FEATURE_RE : Regex = Regex::new(r"(?P<code>\w{3}):(?P<value>[\w#]+)").unwrap();
    }

    RE.captures_iter(input)
        .map(|c| {
            Passport::from(&FEATURE_RE.captures_iter(c.get(0).unwrap().as_str())
                            .map(|c| Feature { code: String::from(c.name("code").unwrap().as_str()), 
                                               value: String::from(c.name("value").unwrap().as_str())})
                            .collect())
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Passport>) -> usize {
    
    input
        .iter()
        .filter(|&p| p.valid())
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Passport>) -> usize {
    
    input
        .iter()
        .filter(|&p| ValidatedPassport::from(p).valid())
        .count()
}

#[cfg(test)]
mod tests {

}