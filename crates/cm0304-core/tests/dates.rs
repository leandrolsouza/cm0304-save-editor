use cm0304_core::database::Date;

#[test]
fn age_uses_calendar_anniversary_across_leap_years() {
    let birth = Date {
        year: 1980,
        day: 60,
    }; // March 1 in leap year
    assert_eq!(birth.month_day(), Some((3, 1)));
    assert_eq!(
        birth.age_on(&Date {
            year: 2003,
            day: 58
        }),
        Some(22)
    ); // Feb 28
    assert_eq!(
        birth.age_on(&Date {
            year: 2003,
            day: 59
        }),
        Some(23)
    ); // March 1
    let leap_birth = Date {
        year: 1980,
        day: 59,
    };
    assert_eq!(leap_birth.month_day(), Some((2, 29)));
    assert_eq!(
        leap_birth.age_on(&Date {
            year: 2004,
            day: 59
        }),
        Some(24)
    );
    assert_eq!(
        Date {
            year: 2003,
            day: 365
        }
        .month_day(),
        None
    );
    assert_eq!(
        Date {
            year: 2100,
            day: 59
        }
        .month_day(),
        Some((3, 1))
    );
    assert_eq!(
        Date { year: 2004, day: 0 }.age_on(&Date {
            year: 2003,
            day: 199
        }),
        None
    );
}
