fn is_leap_year(year: u32) -> bool {
    assert_ne!(year, 0, "year 0 does not exist!");
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[test]
#[cfg(test)]
fn year_1600_is_a_leap_year() {
    assert!(is_leap_year(1600));
}

#[test]
#[cfg(test)]
fn year_1500_is_a_common_year() {
    assert!(!is_leap_year(1500));
}

#[test]
#[cfg(test)]
fn year_2004_is_a_leap_year() {
    assert!(is_leap_year(2004));
}

#[test]
#[cfg(test)]
fn year_2003_is_a_common_year() {
    assert!(!is_leap_year(2003));
}

#[test]
#[cfg(test)]
#[should_panic(expected = "year 0 does not exist!")]
fn year_zero_is_not_a_year() {
    is_leap_year(0);
}

fn num_days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => panic!("unknown month: {month}"),
    }
}

#[test]
#[cfg(test)]
fn feb_has_28_days_on_leap_years() {
    assert_eq!(num_days_in_month(2004, 2), 29);
}

#[test]
#[cfg(test)]
fn feb_has_29_days_on_common_years() {
    assert_eq!(num_days_in_month(2003, 2), 28);
}

#[test]
#[cfg(test)]
fn months_with_31_days() {
    assert_eq!(num_days_in_month(1, 1), 31);
    assert_eq!(num_days_in_month(1, 3), 31);
    assert_eq!(num_days_in_month(1, 5), 31);
    assert_eq!(num_days_in_month(1, 7), 31);
    assert_eq!(num_days_in_month(1, 8), 31);
    assert_eq!(num_days_in_month(1, 10), 31);
    assert_eq!(num_days_in_month(1, 12), 31);
}

#[test]
#[cfg(test)]
fn months_with_30_days() {
    assert_eq!(num_days_in_month(1, 4), 30);
    assert_eq!(num_days_in_month(1, 6), 30);
    assert_eq!(num_days_in_month(1, 9), 30);
    assert_eq!(num_days_in_month(1, 11), 30);
}

#[test]
#[cfg(test)]
#[should_panic(expected = "unknown month: 13")]
fn unknown_month() {
    num_days_in_month(1, 13);
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "Mach",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("unknown month: {month}"),
    }
}

fn main() {
    let mut year = 1;
    let mut month = 1;
    let mut days_max = num_days_in_month(year, month);
    let mut day = 1;
    let mut weekday = 1;

    loop {
        if weekday == 5 && day == 13 {
            println!("Friday, {} 13, {}", month_name(month), year);
        }

        if weekday == 7 {
            weekday = 1;
        } else {
            weekday += 1;
        }

        if day == days_max {
            day = 1;

            if month == 12 {
                month = 1;
                year += 1;
            } else {
                month += 1;
            }

            days_max = num_days_in_month(year, month);
        } else {
            day += 1;
        }
    }
}
