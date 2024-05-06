use crate::time::calendar;

static ISO8601_FORMAT:&str = "%Y-%m-%dT%H:%M:%S%z";
static ISO8601_FRAC_FORMAT:&str = "%Y-%m-%dT%H:%M:%s%z";
static ISO8601_REGEX:&str = 
    "([\\+-]?\\d{4}(?!\\d{2}\\b))\
    ((-?)\
    ((0[1-9]|1[0-2])(\\3([12]\\d|0[1-9]|3[01]))?|W([0-4]\\d|5[0-2])(-?[1-7])?|\
    (00[1-9]|0[1-9]\\d|[12]\\d{2}|3([0-5]\\d|6[1-6])))\
    ([T\\s]\
    ((([01]\\d|2[0-3])((:?)[0-5]\\d)?|24\\:?00)([\\.,]\\d+(?!:))?)?\
    (\\17[0-5]\\d([\\.,]\\d+)?)?([A-I]|[K-Z]|([\\+-])([01]\\d|2[0-3]):?([0-5]\
    \\d)?)?)?)?";

static RFC822_FORMAT:&str = "%w, %e %b %y %H:%M:%S %Z";
static RFC822_REGEX:&str = "(((Mon)|(Tue)|(Wed)|(Thu)|(Fri)|(Sat)|(Sun)), *)?\
                            \\d\\d? +\
                            ((Jan)|(Feb)|(Mar)|(Apr)|(May)|(Jun)|(Jul)|(Aug)|= \
                            Sep)|(Oct)|(Nov)|(Dec)) +\
                            \\d\\d(\\d\\d)? +\
                            \\d\\d:\\d\\d(:\\d\\d)? +\
                            (([+\\-]?\\d\\d\\d\\d)|(UT)|(GMT)|(EST)|(EDT)|= \
                            CST)|(CDT)|(MST)|(MDT)|(PST)|(PDT)|\\w)";

static RFC1123_FORMAT:&str = "%w, %e %b %Y %H:%M:%S %Z";
static RFC1123_REGEX:&str = "(((Mon)|(Tue)|(Wed)|(Thu)|(Fri)|(Sat)|(Sun)), *)?\
                            \\d\\d? +\
                            ((Jan)|(Feb)|(Mar)|(Apr)|(May)|(Jun)|(Jul)|(Aug)|= \
                            Sep)|(Oct)|(Nov)|(Dec)) +\
                            \\d\\d(\\d\\d)? +\
                            \\d\\d:\\d\\d(:\\d\\d)? +\
                            (([+\\-]?\\d\\d\\d\\d)|(UT)|(GMT)|(EST)|(EDT)|= \
                            CST)|(CDT)|(MST)|(MDT)|(PST)|(PDT)|\\w)";

static HTTP_FORMAT:&str = "%w, %d %b %Y %H:%M:%S %Z";
static HTTP_REGEX:&str = "(((Mon)|(Tue)|(Wed)|(Thu)|(Fri)|(Sat)|(Sun)), *)?\
                    \\d\\d? +\
                    ((Jan)|(Feb)|(Mar)|(Apr)|(May)|(Jun)|(Jul)|(Aug)|(Sep)|(Oct)|(Nov)|(Dec)) \
                    +\
                    \\d\\d(\\d\\d)? +\\d\\d:\\d\\d(:\\d\\d)? \
                    ((UT)|(GMT)|(EST)|(EDT)|(CST)|(CDT)|(MST)|(MDT)|(PST)|(PDT)|)?+\
                    (([+\\-]?\\d\\d\\d\\d)?|(UT)|(GMT)|(EST)|(EDT)|(CST)|(CDT)|(MST)|(MDT)|= \
                    PST)|(PDT)|\\w)";

static RFC850_FORMAT:&str = "%W, %e-%b-%y %H:%M:%S %Z";
static RFC850_REGEX:&str = "(((Monday)|(Tuesday)|(Wednesday)|(Thursday)|(Friday)|(Saturday)|(Sunday)|\
            (Mon)|(Tue)|(Wed)|(Thu)|(Fri)|(Sat)|(Sun)), *)?\
            \\d\\d?-((Jan)|(Feb)|(Mar)|(Apr)|(May)|(Jun)|(Jul)|(Aug)|(Sep)|(Oct)|(Nov)\
            |(Dec))-\
            \\d\\d(\\d\\d)? +\\d\\d:\\d\\d(:\\d\\d)? \
            ((UT)|(GMT)|(EST)|(EDT)|(CST)|(CDT)|(MST)|(MDT)|(PST)|(PDT)|)?+\
            (([+\\-]?\\d\\d\\d\\d)?|(UT)|(GMT)|(EST)|(EDT)|(CST)|(CDT)|(MST)|(MDT)|= \
            PST)|(PDT)|\\w)";

static RFC1036_FORMAT:&str = "%W, %e %b %y %H:%M:%S %Z";
static RFC1036_REGEX:&str = "(((Monday)|(Tuesday)|(Wednesday)|(Thursday)|(Friday)|(Saturday)|(Sunday)), \
    *)?\
    \\d\\d? +\
    ((Jan)|(Feb)|(Mar)|(Apr)|(May)|(Jun)|(Jul)|(Aug)|(Sep)|(Oct)|(Nov)|(Dec)) \
    +\
    \\d\\d(\\d\\d)? +\\d\\d:\\d\\d(:\\d\\d)? \
    ((UT)|(GMT)|(EST)|(EDT)|(CST)|(CDT)|(MST)|(MDT)|(PST)|(PDT)|)?+\
    (([+\\-]?\\d\\d\\d\\d)?|(UT)|(GMT)|(EST)|(EDT)|(CST)|(CDT)|(MST)|(MDT)|= \
    PST)|(PDT)|\\w)";

static ASCTIME_FORMAT:&str =  "%w %b %f %H:%M:%S %Y";
static ASCTIME_REGEX:&str =  "((Mon)|(Tue)|(Wed)|(Thu)|(Fri)|(Sat)|(Sun)) +\
                             ((Jan)|(Feb)|(Mar)|(Apr)|(May)|(Jun)|(Jul)|(Aug)|\
                             (Sep)|(Oct)|(Nov)|(Dec)) +\
                             \\d\\d? +\\d\\d:\\d\\d:\\d\\d +(\\d\\d\\d\\d)";

static SORTABLE_FORMAT:&str = "%Y-%m-%d %H:%M:%S";
static SORTABLE_REGEX:&str = "(\\d\\d\\d\\d-\\d\\d-\\d\\d \\d\\d:\\d\\d:\\d\\d)";


static FORMAT_LIST:[&str;9] = [
        ISO8601_FORMAT, 
        ISO8601_FRAC_FORMAT,
        RFC822_FORMAT,  
        RFC1123_FORMAT,
        HTTP_FORMAT,    
        RFC850_FORMAT,
        RFC1036_FORMAT, 
        ASCTIME_FORMAT,
        SORTABLE_FORMAT];    

static REGEX_LIST:[&str;9] = [
    ISO8601_REGEX,
    ISO8601_REGEX, //->ISO8601_FRAC_FORMAT,
    RFC822_REGEX,  
    RFC1123_REGEX,
    HTTP_REGEX,    
    RFC850_REGEX,
    RFC1036_REGEX, 
    ASCTIME_REGEX,
    SORTABLE_REGEX
];

static Zones:[(&str,i32);34] = [
    ("Z", 0),
    ("UT", 0),
    ("GMT", 0),
    ("BST", 1 * 3600),
    ("IST", 1 * 3600),
    ("WET", 0),
    ("WEST", 1 * 3600),
    ("CET", 1 * 3600),
    ("CEST", 2 * 3600),
    ("EET", 2 * 3600),
    ("EEST", 3 * 3600),
    ("MSK", 3 * 3600),
    ("MSD", 4 * 3600),
    ("NST", -3 * 3600 - 1800),
    ("NDT", -2 * 3600 - 1800),
    ("AST", -4 * 3600),
    ("ADT", -3 * 3600),
    ("EST", -5 * 3600),
    ("EDT", -4 * 3600),
    ("CST", -6 * 3600),
    ("CDT", -5 * 3600),
    ("MST", -7 * 3600),
    ("MDT", -6 * 3600),
    ("PST", -8 * 3600),
    ("PDT", -7 * 3600),
    ("AKST", -9 * 3600),
    ("AKDT", -8 * 3600),
    ("HST", -10 * 3600),
    ("AEST", 10 * 3600),
    ("AEDT", 11 * 3600),
    ("ACST", 9 * 3600 + 1800),
    ("ACDT", 10 * 3600 + 1800),
    ("AWST", 8 * 3600),
    ("AWDT", 9 * 3600)
];


static WEEKDAY_NAMES:[&str;7] = [
    "Sunday",   "Monday", "Tuesday", "Wednesday",
    "Thursday", "Friday", "Saturday"];

static MONTH_NAMES:[&str;12] = [
    "January", "February", "March",     "April",   "May",      "June",
    "July",    "August",   "September", "October", "November", "December"];


//------- TarDateTime -------
pub struct TarDateTime {
    m_year:u32,
    m_month:u32,
    m_day:u32,
    m_hour:u32,
    m_minute:u32,
    m_second:u32,
    m_millisecond:u32,
    m_microsecond:u32,
    m_day_of_week:u32,  //[0,6]
    m_day_of_month:u32, //[1,31]
    m_day_of_year:u32,  //[0,365]
    m_tzd:i32,
}

const ASSIIC_0:u8 = '0' as u8;
const ASSIIC_9:u8 = '9' as u8;

const ASSIIC_W:u8 = 'w' as u8;
const ASSIIC_W_UPPER:u8 = 'W' as u8;
const ASSIIC_SPACE:u8 = ' ' as u8;

const ASSIIC_A:u8 = 'a' as u8;
const ASSIIC_A_UPPER:u8 = 'A' as u8;
const ASSIIC_Z:u8 = 'z' as u8;
const ASSIIC_Z_UPPER:u8 = 'Z' as u8;

const ASSIIC_B:u8 = 'b' as u8;
const ASSIIC_B_UPPER:u8 = 'B' as u8;

const ASSIIC_D:u8 = 'd' as u8;
const ASSIIC_E:u8 = 'e' as u8;
const ASSIIC_F:u8 = 'f' as u8;

const ASSIIC_M:u8 = 'm' as u8;
const ASSIIC_M_UPPER:u8 = 'M' as u8;
const ASSIIC_N:u8 = 'n' as u8;
const ASSIIC_O:u8 = 'o' as u8;

const ASSIIC_Y:u8 = 'y' as u8;
const ASSIIC_Y_UPPER:u8 = 'Y' as u8;

const ASSIIC_R:u8 = 'r' as u8;

const ASSIIC_H:u8 = 'h' as u8;
const ASSIIC_H_UPPER:u8 = 'H' as u8;

const ASSIIC_S:u8 = 's' as u8;
const ASSIIC_S_UPPER:u8 = 'S' as u8;

const ASSIIC_I:u8 = 'i' as u8;

const ASSIIC_C:u8 = 'c' as u8;

const ASSIIC_F_UPPER:u8 = 'F' as u8;

const ASSIIC_PLUS:u8 = '+' as u8;
const ASSIIC_MINUS:u8 = '-' as u8;

const ASSIIC_COTON:u8 = ':' as u8;
const ASSIIC_PERCENT:u8 = '%' as u8;

impl TarDateTime {
    //pub function
    pub fn new()->Self {
        TarDateTime {
            m_year:0,
            m_month:0,
            m_day:0,
            m_hour:0,
            m_minute:0,
            m_second:0,
            m_millisecond:0,
            m_microsecond:0,
            m_day_of_week:0,
            m_day_of_month:0, 
            m_day_of_year:0,
            m_tzd:0,
        }
    }

    pub fn year(&mut self,year:u32)->&mut TarDateTime {
        self.m_year = year;
        self
    }

    pub fn get_year(&self)->u32 {
        self.m_year
    }

    pub fn month(&mut self,month:u32)->&mut TarDateTime {
        self.m_month = month;
        self
    }

    pub fn get_month(&self)->u32 {
        self.m_month
    }

    pub fn day(&mut self,day:u32)->&mut TarDateTime {
        self.m_day = day;
        self
    }

    pub fn get_day(&self)->u32 {
        self.m_day
    }

    pub fn hour(&mut self,hour:u32)->&mut TarDateTime {
        self.m_hour = hour;
        self
    }

    pub fn get_hour(&self)->u32 {
        self.m_hour
    }

    pub fn minute(&mut self,minute:u32)->&mut TarDateTime {
        self.m_minute = minute;
        self
    }

    pub fn get_minute(&self)->u32 {
        self.m_minute
    }

    pub fn second(&mut self,second:u32)->&mut TarDateTime {
        self.m_second = second;
        self
    }

    pub fn get_second(&self)->u32 {
        self.m_second
    }

    pub fn msecond(&mut self,msecond:u32)->&mut TarDateTime {
        self.m_millisecond = msecond;
        self
    }

    pub fn get_msecond(&self)->u32 {
        self.m_millisecond
    }

    pub fn import_iso8601(&mut self,date_str:&str) {
        self.parse(ISO8601_FORMAT, String::from(date_str));
    }

    pub fn dump(&self) {
        println!("year is {}",self.m_year);
        println!("month is {}",self.m_month);
        println!("day is {}",self.m_day);
        println!("hour is {}",self.m_hour);
        println!("minute is {}",self.m_minute);
        println!("second is {}",self.m_second);
    }
    //private function
    fn is_alphabetic(&self,c:u8)->bool {
        (c <= ASSIIC_Z_UPPER && c >= ASSIIC_Z) ||
        (c <= ASSIIC_A_UPPER && c >= ASSIIC_A)
    }

    fn is_digit(&self,c:u8)->bool {
        c <= ASSIIC_9 && c >= ASSIIC_0
    }

    fn is_space(&self,c:u8)-> bool {
        c == ASSIIC_SPACE
    }

    fn skip_junk(&self,start:&mut usize,end:usize,date_str:&[u8]) {
        while *start != end && !self.is_digit(date_str[*start]) {
            *start += 1;
        }
    }

    fn parse_number_with_len(&self,start:&mut usize,end:usize,date_str:&[u8],len:u8)->u32 {
        let l_start = *start;
        let mut l_len = 0;

        while *start < end && l_len < len && self.is_digit(date_str[*start]) {
            *start += 1;
            l_len += 1;
        }

        let val = String::from_utf8_lossy(&date_str[l_start..*start]);
        val.parse::<u32>().unwrap()
    }

    fn parse_number(&self,start:&mut usize,end:usize,date_str:&[u8])->u32 {
        let l_start = *start;
        while *start < end && self.is_digit(date_str[*start]) {
            *start += 1
        }

        let val = String::from_utf8_lossy(&date_str[l_start..*start]);
        val.parse::<u32>().unwrap()
    }

    fn parse_year(&mut self,start:&mut usize,end:usize,date_str:&[u8],len:u8)->u8 {
        let mut l_start = *start;
        let mut count:u8 = 0;
        while *start < end && !self.is_space(date_str[*start]) && self.is_digit(date_str[*start]) && count < len {
            count += 1;
            *start += 1;
        }

        let year_str = String::from_utf8_lossy(&date_str[l_start..*start]);
        self.m_year = year_str.parse::<u32>().unwrap();
        count
    }

    fn parse_tzd(&mut self,start:&mut usize,end:usize,date_str:&[u8]) {
        while *start < end && self.is_space(date_str[*start]) {
            *start += 1;
        }

        let mut tzd = 0;

        if *start < end {
            if self.is_alphabetic(date_str[*start]) {
                let mut l_start = *start;
                while *start < end && self.is_alphabetic(date_str[*start]) {
                    *start += 1;
                }

                let l_zone = String::from_utf8_lossy(&date_str[l_start..*start]);
                for zone in Zones {
                    if l_zone == zone.0 {
                        tzd = zone.1;
                        break;
                    }
                }
            }

            if *start < end {
                let mut sign:i32 = 0;
                match date_str[*start] {
                    ASSIIC_PLUS => {
                        sign = 1;
                    },
                    ASSIIC_MINUS => {
                        sign = -1;
                    },
                    _=> {
                        return;
                    }
                }
                *start += 1;
                let hour = self.parse_number_with_len(start, end, date_str, 2);
                if *start < end && date_str[*start] == ASSIIC_COTON {
                    *start += 1;
                }

                let minute = self.parse_number_with_len(start, end, date_str, 2);
                self.m_tzd = sign * (hour * 3600 + minute * 60) as i32;
            }
        }
    }

    fn parse_month(&self,start:usize,end:usize,date_str:&[u8])->calendar::Month {
        let month = String::from_utf8_lossy(&date_str[start..end]).trim().to_uppercase();
        match month.as_str() {
            "january"=> {
                return calendar::Month::January;
            },
            "february"=> {
                return calendar::Month::February;
            },
            "march"=>{
                return calendar::Month::March;
            },
            "april"=>{
                return calendar::Month::April;
            },
            "may"=>{
                return calendar::Month::May;
            },
            "june"=>{
                return calendar::Month::June;
            },
            "july"=>{
                return calendar::Month::July;
            },
            "august"=>{
                return calendar::Month::August;
            },
            "september"=>{
                return calendar::Month::September;
            }, 
            "october"=>{
                return calendar::Month::October;
            }, 
            "november"=>{
                return calendar::Month::November;
            },
            "december"=>{
                return calendar::Month::December;
            }
            _=>{}
        }
        calendar::Month::Err
    }

    fn parse_day_of_week(&self,start:usize,end:usize,date_str:&[u8])->calendar::WeekDay {
        let day_of_week = String::from_utf8_lossy(&date_str[start..end]).trim().to_uppercase();
        match day_of_week.as_str() {
            "MON"=>{
                return calendar::WeekDay::Monday;
            },
            "TUE"=>{
                return calendar::WeekDay::Tuesday;
            },
            "WED"=>{
                return calendar::WeekDay::Wednesday;
            },
            "THU"=>{
                return calendar::WeekDay::Thursday;
            },
            "FRI"=>{
                return calendar::WeekDay::Friday;
            },
            "SAT"=>{
                return calendar::WeekDay::Saturday;
            },
            "SUN"=>{
                return calendar::WeekDay::Sunday;
            }
            _=>{}
        }

        calendar::WeekDay::Err
    }

    fn parse_hour_with_am_pm(&mut self,start:&mut usize,end:usize,date_str:&[u8]) {
        while *start < end && self.is_space(date_str[*start]) {
            *start += 1;
        }

        let l_start = *start;
        while *start < end && self.is_alphabetic(date_str[*start]){
            *start += 1;
        }

        let am_pm = String::from_utf8_lossy(&date_str[l_start..*start]);
        if am_pm == "AM" && self.m_hour == 12 {
            self.m_hour = 0;
        } else if am_pm == "PM" && self.m_hour < 12 {
            self.m_hour += 12;
        }
    }

    fn parse(&mut self,fmt:&str,date_str:String) {
        let mut start = 0;
        let end = date_str.len();

        let mut start_f = 0;
        let end_f = fmt.len();

        let fmt_slice = fmt.as_bytes();
        let str_slice = date_str.as_bytes();

        while start < end && start_f < end_f {
            if fmt_slice[start_f] == '%' as u8 {
                start_f += 1;
                if start_f != end_f {
                    match fmt_slice[start_f] {
                        ASSIIC_W | ASSIIC_W_UPPER => {
                            let l_start = start;
                            while start != end && str_slice[start] != ASSIIC_SPACE {
                                start += 1;
                            }

                            while start != end && self.is_alphabetic(str_slice[start]) {
                                start += 1
                            }
                            self.m_day_of_week = self.parse_day_of_week(l_start,start,&str_slice) as u32;
                        },
                        ASSIIC_B | ASSIIC_B_UPPER => {
                            let l_start = start;
                            while start != end && str_slice[start] != ASSIIC_SPACE {
                                start += 1;
                            }

                            while start != end && self.is_alphabetic(str_slice[start]) {
                                start += 1
                            }
                            self.m_month = self.parse_month(l_start, start,&str_slice) as u32;
                        },
                        ASSIIC_D | ASSIIC_E |ASSIIC_F => {
                            self.skip_junk(&mut start,end,&str_slice);
                            self.m_day = self.parse_number_with_len(&mut start, end, &str_slice, 2);
                        },
                        ASSIIC_M | ASSIIC_N |ASSIIC_O => {
                            self.skip_junk(&mut start,end,&str_slice);
                            self.m_month = self.parse_number_with_len(&mut start, end, str_slice, 2);
                            self.m_month -= 1;
                        },
                        ASSIIC_Y | ASSIIC_Y_UPPER => {
                            self.skip_junk(&mut start, end, str_slice);

                            let len = self.parse_year(&mut start,end,&str_slice,4);
                            if len == 2 {
                                if self.m_year >= 69 {
                                    self.m_year += 1900;
                                } else {
                                    self.m_year += 2000;
                                }
                            }
                        },
                        ASSIIC_R => {
                            self.skip_junk(&mut start, end, str_slice);
                            self.m_year = self.parse_number(&mut start, end, str_slice);
                            if self.m_year < 1000 {
                                if self.m_year >= 69 {
                                    self.m_year += 1900;
                                } else {
                                    self.m_year += 2000;
                                }
                            }
                        },
                        ASSIIC_H | ASSIIC_H_UPPER => {
                            self.skip_junk(&mut start, end, str_slice);
                            self.m_hour = self.parse_number_with_len(&mut start, end, str_slice, 2);
                        },
                        ASSIIC_A | ASSIIC_A_UPPER => {
                            self.parse_hour_with_am_pm(&mut start, end, str_slice);
                        },
                        ASSIIC_M_UPPER => {
                            self.skip_junk(&mut start, end, str_slice);
                            self.m_minute = self.parse_number_with_len(&mut start, end, str_slice, 2);
                        },
                        ASSIIC_S | ASSIIC_S_UPPER => {
                            self.skip_junk(&mut start, end, str_slice);
                            self.m_second = self.parse_number_with_len(&mut start, end, str_slice, 2);
                            //TODO
                        },
                        ASSIIC_I => {
                            self.skip_junk(&mut start, end, str_slice);
                            self.m_millisecond = self.parse_number_with_len(&mut start, end, str_slice, 3);
                        },
                        ASSIIC_C => {
                            self.skip_junk(&mut start, end,str_slice);
                            self.m_millisecond = self.parse_number_with_len(&mut start, end, str_slice, 1);
                            self.m_millisecond *= 100;
                        },
                        ASSIIC_F_UPPER => {
                            //TODO
                        },
                        ASSIIC_Z |ASSIIC_Z_UPPER => {
                            self.parse_tzd(&mut start, end, str_slice);
                        }
                        _=>{}
                    }
                }
            } else {
                start_f += 1;
            }

        }
    }

    //format
    fn format_num(&self,val:u32)->String {
        format!("{}",val)
    }

    fn format_num_width2(&self,val:u32,fillzero:bool)->String {
        if fillzero {
            return format!("{:0>2}",val);
        }

        return format!("{:2}",val);
    }

    fn format_num_width3(&self,val:u32,fillzero:bool)->String {
        if fillzero {
            return format!("{:0>3}",val);
        }

        return format!("{:3}",val);
    }

    fn format_num_width4(&self,val:u32,fillzero:bool)->String {
        if fillzero {
            return format!("{:0>4}",val);
        }

        return format!("{:4}",val);
    }

    fn format_num_width6(&self,val:usize,fillzero:bool)->String {
        if fillzero {
            return format!("{:0>6}",val);
        }

        return format!("{:6}",val);
    }

    fn hour_am_pm(&self)->u32 {
        if self.m_hour < 1 {
            return 12;
        } else if self.m_hour > 12 {
            return self.m_hour - 12;
        } 
        
        return self.m_hour;
    }

    pub fn to_string_ISO8601(&self)->String {
        self.format(ISO8601_FORMAT)
    }

    fn to_tzd_iso(&self,time_zone_diff:i32)->String {
        let mut zone_str = String::from("");
        if time_zone_diff != 0xFFFF {
            if time_zone_diff > 0 {
                zone_str += "+";
                zone_str += &self.format_num_width2(time_zone_diff as u32/3600, true);
                zone_str += ":";
                zone_str += &self.format_num_width2((time_zone_diff as u32 %3600)/60, true);
            } else {
                zone_str += "-";
                let new_diff = -time_zone_diff;
                zone_str += &self.format_num_width2(new_diff as u32/3600, true);
                zone_str += ":";
                zone_str += &self.format_num_width2((new_diff as u32 %3600)/60, true);
            }
        } else {
            zone_str += "Z";
        }
        zone_str
    }

    fn to_tzd_rfc(&self,time_zone_diff:i32)->String {
        let mut zone_str = String::from("");
        if time_zone_diff != 0xFFFF {
            if time_zone_diff > 0 {
                zone_str += "+";
                zone_str += &self.format_num_width2(time_zone_diff as u32/3600, true);
                zone_str += ":";
                zone_str += &self.format_num_width2((time_zone_diff as u32 %3600)/60, true);
            } else {
                zone_str += "-";
                let new_diff = -time_zone_diff;
                zone_str += &self.format_num_width2(new_diff as u32/3600, true);
                zone_str += ":";
                zone_str += &self.format_num_width2((new_diff as u32 %3600)/60, true);
            }
        } else {
            zone_str += "GMT";
        }
        zone_str
    }

    fn format(&self,format:&str)->String {
        let mut start = 0;
        let end = format.len();
        let str_slice = format.as_bytes();

        let mut result = String::from("");

        while start < end {
            if str_slice[start] == ASSIIC_PERCENT {
                start += 1;
                let t = str_slice[start].to_string();
                println!("str_slice[start] is {}",t);
                match str_slice[start] {
                    ASSIIC_W => {
                        let day_of_week = super::calendar::TarCalendar::day_of_week(self.m_year, self.m_month, self.m_day);
                        result += &(WEEKDAY_NAMES[day_of_week as usize][0..3]);
                    },
                    ASSIIC_W_UPPER => {
                        let day_of_week = super::calendar::TarCalendar::day_of_week(self.m_year, self.m_month, self.m_day);
                        result += &WEEKDAY_NAMES[day_of_week as usize];
                    },
                    ASSIIC_B => {
                        result += &(MONTH_NAMES[self.m_month as usize][0..3]);
                    },
                    ASSIIC_B_UPPER => {
                        result += &MONTH_NAMES[self.m_month as usize];
                    },
                    ASSIIC_D => {
                        result += &self.format_num_width2(self.m_day, true);
                    },
                    ASSIIC_E => {
                        result += &self.format_num(self.m_day_of_month);
                    },
                    ASSIIC_F => {
                        result += &self.format_num_width2(self.m_day_of_month, false);
                    },
                    ASSIIC_M => {
                        result += &self.format_num_width2(self.m_month + 1, true);
                    },
                    ASSIIC_N => {
                        result += &self.format_num(self.m_month + 1);
                    },
                    ASSIIC_O => {
                        result += &self.format_num_width2(self.m_month + 1, false);
                    },
                    ASSIIC_Y => {
                        result += &self.format_num_width2(self.m_year % 100, true);
                    },
                    ASSIIC_Y_UPPER => {
                        result += &self.format_num_width4(self.m_year, true);
                    },
                    ASSIIC_H => {
                        result += &self.format_num_width2(self.hour_am_pm(), true);
                    },
                    ASSIIC_H_UPPER => {
                        result += &self.format_num_width2(self.m_hour, true);
                    },
                    ASSIIC_A => {
                        if self.m_hour > 12 {
                            result += "am";
                        } else {
                            result += "pm";
                        }
                    },
                    ASSIIC_A_UPPER => {
                        if self.m_hour > 12 {
                            result += "AM";
                        } else {
                            result += "PM";
                        }
                    },
                    ASSIIC_M_UPPER => {
                        result += &self.format_num_width2(self.m_minute, true);
                    },
                    ASSIIC_S => {
                        result += &self.format_num_width2(self.m_second, true);
                        result += ".";
                        result += &self.format_num_width6(self.m_millisecond as usize *1000 + self.m_microsecond as usize,true);
                    },
                    ASSIIC_S_UPPER => {
                        result += &self.format_num_width2(self.m_second, true);
                    },
                    ASSIIC_I => {
                        result += &self.format_num_width3(self.m_millisecond,true);
                    },
                    ASSIIC_C => {
                        result += &self.format_num(self.m_millisecond/100);
                    },
                    ASSIIC_F_UPPER => {
                        result += &self.format_num_width6(self.m_millisecond as usize, true);
                    },
                    ASSIIC_Z => {
                        if self.m_tzd != 0 {
                            result += &self.to_tzd_iso(self.m_tzd);
                        } else {
                            result += &self.to_tzd_iso(0xFFFF);
                        }
                    },
                    ASSIIC_Z_UPPER => {
                        if self.m_tzd != 0 {
                            result += &self.to_tzd_rfc(self.m_tzd);
                        } else {
                            result += &self.to_tzd_rfc(0xFFFF);
                        }
                    },
                    _=> {
                        result += &str_slice[start].to_string();
                    }
                }
                start += 1;
            } else {
                result = result + &String::from_utf8_lossy(&[str_slice[start]]);
                start += 1;
            }
        }

        return result;
    }
}