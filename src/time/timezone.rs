use libc::{tm, time_t, localtime_r};

pub struct TarTimeZone {}

impl TarTimeZone {
    pub fn current()->u32 {
        unsafe {
            let mut p_tm_time: tm = std::mem::uninitialized();
            let time_value: time_t = 0; // 示例时间值
            localtime_r(&time_value, &mut p_tm_time);
            if p_tm_time.tm_hour > 12 {
                return p_tm_time.tm_hour as u32 - 24;
            } else {
                return p_tm_time.tm_hour as u32;
            }
        }
    }
}