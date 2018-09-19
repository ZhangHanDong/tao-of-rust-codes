#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?x)
        (?P<year>\d{4})-  # the year
        (?P<month>\d{2})- # the month
        (?P<day>\d{2})   # the day
    ").unwrap();
   static ref EMAIL_RE: Regex = Regex::new(r"(?x)
       ^\w+@(?:gmail|163|qq)\.(?:com|cn|com\.cn|net)$
   ").unwrap();
}

fn regex_date(text: &str) -> regex::Captures {
   RE.captures(text).unwrap()
}

fn regex_email(text: &str) -> bool {
   EMAIL_RE.is_match(text)
}

fn main() {
   let caps = regex_date("2018-01-01");
   assert_eq!("2018", &caps["year"]);
   assert_eq!("01", &caps["month"]);
   assert_eq!("01", &caps["day"]);
   let after = RE.replace_all("2018-01-01", "$month/$day/$year");
   assert_eq!(after, "01/01/2018");
   assert!(regex_email("alex@gmail.com"), true);
   assert_eq!(regex_email("alex@gmail.cn.com"), false);
}