extern crate time;
#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate winapi;

use time::Tm;

#[derive(Copy, Clone, Debug)]
pub struct MsDosDateTime {
    pub datepart: u16,
    pub timepart: u16,
}

impl MsDosDateTime {
    pub fn new(date: u16, time: u16) -> MsDosDateTime {
        MsDosDateTime {
            datepart: date,
            timepart: time,
        }
    }
}

pub trait TmMsDosExt {
    fn to_msdos(&self) -> MsDosDateTime;
    fn from_msdos(MsDosDateTime) -> Option<Self>;
}

impl TmMsDosExt for Tm {
    fn to_msdos(&self) -> MsDosDateTime {
        sys::tm_to_msdos(self)
    }
    fn from_msdos(ms: MsDosDateTime) -> Option<Self> {
        sys::msdos_to_tm(ms)
    }
}

#[cfg(not(windows))]
mod sys {
    use super::MsDosDateTime;
    use time::Tm;

    pub fn msdos_to_tm(ms: MsDosDateTime) -> Option<Tm> {
        unimplemented!()
    }

    pub fn tm_to_msdos(tm: &Tm) -> MsDosDateTime {
        unimplemented!()
    }
}

#[cfg(windows)]
mod sys {
    use super::MsDosDateTime;
    use time::Tm;

    pub fn msdos_to_tm(ms: MsDosDateTime) -> Option<Tm> {
        unimplemented!()
    }

    pub fn tm_to_msdos(tm: &Tm) -> MsDosDateTime {
        unimplemented!()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use time::{self, Tm};

    fn check_date(input: Tm, day: i32, month: i32, year: i32) {
        assert_eq!(input.tm_mday, day);
        assert_eq!(input.tm_mon + 1, month);
        assert_eq!(input.tm_year + 1900, year);
    }

    fn check_time(input: Tm, hour: i32, minute: i32, second: i32) {
        assert_eq!(input.tm_hour, hour);
        assert_eq!(input.tm_min, minute);
        assert_eq!(input.tm_sec, second);
    }

    #[test]
    fn dos_zero() {
        // The 0 date is not a correct msdos date
        assert!(Tm::from_msdos(MsDosDateTime::new(0, 0)).is_none());
    }

    #[test]
    fn dos_smallest() {
        // This is the actual smallest date possible
        let tm = Tm::from_msdos(MsDosDateTime::new(0, 0b100001)).unwrap();
        check_date(tm, 1, 1, 1980);
        check_time(tm, 0, 0, 0);
    }

    #[test]
    fn dos_today() {
        let tm = Tm::from_msdos(MsDosDateTime::new(0b01001_100000_10101, 0b0100011_0110_11110)).unwrap();
        check_date(tm, 30, 6, 2015);
        check_time(tm, 9, 32, 42);
    }

    #[test]
    fn zero_dos() {
        let tm = Tm {
            tm_year: 80,
            tm_mon: 0,
            tm_mday: 1,
            tm_hour: 0,
            tm_min: 0,
            tm_sec: 0,
            ..time::empty_tm()
        };
        let ms = tm.to_msdos();
        assert_eq!(ms.datepart, 0b100001);
        assert_eq!(ms.timepart, 0);
    }

    #[test]
    fn today_dos() {
        let tm = Tm {
            tm_year: 115,
            tm_mon: 5,
            tm_mday: 30,
            tm_hour: 9,
            tm_min: 32,
            tm_sec: 42,
            ..time::empty_tm()
        };
        let ms = tm.to_msdos();
        assert_eq!(ms.datepart, 0b0100011_0110_11110);
        assert_eq!(ms.timepart, 0b01001_100000_10101);
    }
}
