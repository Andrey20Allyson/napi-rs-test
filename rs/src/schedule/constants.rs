pub const U8_NULL: u8 = 255;

pub const NUM_OF_DAYS_PER_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const WORKER_PER_DUTY: usize = 3;
pub const DUTY_PER_DAY: usize = 4;
pub const DAY_LIMIT: usize = 32;
pub const DUTY_QUANTITY: usize = DAY_LIMIT * DUTY_PER_DAY;
pub const WORKER_LIMIT: usize = 34;
