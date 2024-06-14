use crate::lang::system::current_millis;
use crate::time::datetime::TarDateTime;

use super::timezone::TarTimeZone;

pub enum WeekDay {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Err
}

pub enum Month {
    January = 0,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    Err
}

pub enum Field {
    Year = 0,
    Month,
    DayOfWeek,
    DayOfMonth,
    DayOfYear,
    Hour,
    Minute,
    Second,
    MilliSecond,
}

const NO_LEAP_MONTH_DAYS:[u64;12] = [31,28,31,30,31,30,31,31,30,31,30,31];
const LEAP_MONTH_DAYS:[u64;12] = [31,29,31,30,31,30,31,31,30,31,30,31];

const SecondMillsecond:u64 = 1000;
const MinuteMillsecond:u64 = 60 * SecondMillsecond;
const HourMillsecond:u64 = 60 * MinuteMillsecond;
const DayMillsecond:u64 = 24 * HourMillsecond;

pub struct TarCalendar{
    m_date_time:TarDateTime,
    m_raw_time:u64
}

impl TarCalendar {
    pub fn new(millsecond:u64)->Self {
        let mut calendar = TarCalendar {
            m_date_time:TarDateTime::new(),
            m_raw_time:0
        };

        if millsecond != 0 {
            calendar.update_current(millsecond + TarTimeZone::current() as u64 *HourMillsecond);
        } else {
            calendar.update_current(current_millis() + TarTimeZone::current() as u64 *HourMillsecond);
        }

        calendar
    }

    pub fn set(&mut self,field:Field,value:u32) {
        match field {
            Field::Year => {
                let year = self.m_date_time.get_year();
                self.add(field, (value as i64 - year as i64) as i32);
            },
            Field::Month => {
                let month = self.m_date_time.get_month();
                self.add(field, (value as i64 - month as i64) as i32);
            },
            Field::DayOfMonth => {
                let day = self.m_date_time.get_day();
                self.add(field, (value as i64 - day as i64) as i32);
            },
            Field::Hour => {
                let hour = self.m_date_time.get_hour();
                self.add(field, (value as i64 - hour as i64) as i32);
            },
            Field::Minute => {
                let minute: u32 = self.m_date_time.get_minute();
                self.add(field, (value as i64 - minute as i64) as i32);
            },
            Field::Second => {
                let second: u32 = self.m_date_time.get_second();
                self.add(field, (value as i64 - second as i64) as i32);
            },
            Field::MilliSecond => {
                let msecond: u32 = self.m_date_time.get_msecond();
                self.add(field, (value as i64 - msecond as i64) as i32);
            },
            _=> {}
        }
    }

    pub fn get_date_time(&self)->&TarDateTime {
        return &self.m_date_time;
    }

    pub fn add(&mut self,field:Field,value:i32) {
        match field {
            Field::Year => {
                let mut year = self.m_date_time.get_year();

                let mut l_value = 0;
                if value > 0 {
                    l_value = value as u32;
                } else {
                    l_value = (-value) as u32;
                }

                while l_value != 0 {
                    if TarCalendar::is_leap(year as u64 + 1) {
                        self.m_raw_time += 366 * DayMillsecond;
                    } else {
                        self.m_raw_time += 365 * DayMillsecond;
                    }

                    if value > 0 {
                        year += 1;
                    } else {
                        year -= 1;
                    }
                    
                    l_value -= 1;
                }
                
            },
            Field::Month => {
                let mut year = self.m_date_time.get_year();
                let mut month = self.m_date_time.get_month();

                let mut l_value = 0;
                if value > 0 {
                    l_value = value as u32;
                } else {
                    l_value = (-value) as u32;
                }

                let mut day:u64 = 0;
                while l_value != 0 {
                    if TarCalendar::is_leap(year as u64) {
                        day += LEAP_MONTH_DAYS[month as usize];
                    } else {
                        day += NO_LEAP_MONTH_DAYS[month as usize];
                    }
                    month += 1;

                    if month > Month::December as u32 {
                        if value > 0 {
                            year += 1;
                        } else {
                            year -= 1;
                        }
                        
                        month = Month::January as u32;
                    }

                    l_value -= 1;
                    if value > 0 {
                        self.m_raw_time += (day as u64)*DayMillsecond;
                    } else {
                        self.m_raw_time -= (day as u64) *DayMillsecond;
                    }
                }
            },
            Field::DayOfMonth => {
                if value > 0 {
                    self.m_raw_time += (value as u64)*DayMillsecond;
                } else {
                    self.m_raw_time += (-value) as u64 *DayMillsecond;
                }
            },
            Field::Hour => {
                if value > 0 {
                    self.m_raw_time += (value as u64)*HourMillsecond;
                } else {
                    self.m_raw_time += (-value) as u64 *HourMillsecond;
                }
            },
            Field::Minute => {
                if value > 0 {
                    self.m_raw_time += (value as u64)*MinuteMillsecond;
                } else {
                    self.m_raw_time += (-value) as u64 *MinuteMillsecond;
                }
            },
            Field::Second => {
                if value > 0 {
                    self.m_raw_time += (value as u64)*SecondMillsecond;
                } else {
                    self.m_raw_time += (-value) as u64 *SecondMillsecond;
                }
            },
            Field::MilliSecond=> {
                if value > 0 {
                    self.m_raw_time += value as u64;
                } else {
                    self.m_raw_time += (-value) as u64;
                }
            },
            _=> {
                //TODO
            }
        }
        self.update_current(self.m_raw_time);
    }

    fn update_current(&mut self,millsecond:u64) {
        let mut current = millsecond;
        self.m_raw_time = millsecond;

        if current == 0 {
            current = current_millis();
        }

        let l_mill_second: u64 = current%1000;
        current = current/1000;

        let l_second = current % 60;
        current = current / 60;

        let l_minute = current % 60;
        current = current / 60;

        let l_hour = current%24;
        current = current/24;

        let mut l_year = 1970;
        while current >= 365 {
            current -= 365;
            if TarCalendar::is_leap(l_year) {
                if current >= 1 {
                    current -= 1
                } else {
                    break;
                }
            }
            l_year += 1;
        }

        let mut l_month:u8 = 0;

        let mut month_list:[u64;12]  = [0;12];
        if TarCalendar::is_leap(l_year) {
            month_list = LEAP_MONTH_DAYS;
        } else {
            month_list = NO_LEAP_MONTH_DAYS;
        }

        for n in month_list {
          if current >= n {current -= n;}
          else {break;}
          l_month += 1;
        }

        let l_day = current;

        self.m_date_time.second(l_second as u32);
        self.m_date_time.year(l_year as u32);
        self.m_date_time.month(l_month as u32);
        self.m_date_time.day(l_day as u32);
        self.m_date_time.hour(l_hour as u32);
        self.m_date_time.minute(l_minute as u32);
        self.m_date_time.msecond(l_mill_second as u32);
    }

    pub fn dump(&self) {
        self.m_date_time.dump();
    }

    pub fn is_leap(year:u64)->bool {
        (year % 4 == 0 && year % 100 != 0) || year % 4 == 400
    }

    pub fn day_of_week(year:u32,month:u32,day:u32)->u32 {
        let mut y = year;
        let mut m = month;
        let mut d = day;

        m += 1;
        d += 1;

        if m == 1 || m == 2 {
            m += 12;
            y -= 1;
        }

        return (d + 2 * m + 3 * (m + 1) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
    }
}