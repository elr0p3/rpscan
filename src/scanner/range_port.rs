use std::{
    num::ParseIntError,
    str::FromStr
};


pub const SEP: &'static str = "-";
const DEF_THREADS: u8 = 1;


#[derive(Debug, Clone, Copy)]
pub struct RangePorts {
    low: u16,
    high: u16,
    num: u16,
    threads_to_use: u8,
}


enum MergeMethod {
    SelfContainsOther,
    OtherContainsSelf,
    SelfIsLower,
    SelfIsHigher,

    SelfLowOtherHigh,
    SelfHighOtherLow,

    Wrong,
}


impl FromStr for RangePorts {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ports = String::new();

        if s.starts_with(SEP) {
            ports += "1";
            ports += s;
        } else if s.ends_with(SEP) {
            ports += s;
            ports += "65535";   // u16::MAX
        } else {
            ports += s;
        }

        let nums: Vec<&str> = ports.split(SEP).collect();
        let mut vnum: Vec<u16> = vec![];
        for pn in nums {
            vnum.push(pn.parse()?);
        }

        if vnum[0] > vnum[1] {
            vnum[0] ^= vnum[1];
            vnum[1] ^= vnum[0];
            vnum[0] ^= vnum[1];
        }

        Ok(Self{
            low: vnum[0],
            high: vnum[1],
            num: vnum[1] - vnum[0] + 1,
            threads_to_use: DEF_THREADS,
        })
    }
}

impl RangePorts {

    #[allow(dead_code)]
    pub fn new (mut low: u16, mut high: u16) -> Self {
        if low > high {
            low ^= high;
            high ^= low;
            low ^= high;
        }

        Self {
            low,
            high,
            num: high - low + 1,
            threads_to_use: DEF_THREADS,
        }
    }

    pub fn get_low (&self) -> u16 {
        self.low
    }

    #[allow(dead_code)]
    pub fn get_high (&self) -> u16 {
        self.high
    }

    pub fn get_num (&self) -> u16 {
        self.num
    }

    pub fn same_pair_port (&self) -> bool {
        self.low == self.high
    }

    #[allow(dead_code)]
    pub fn get_threads_to_use (&self) -> u8 {
        self.threads_to_use
    }

    pub fn set_threads_to_use (&mut self, t: u8) {
        self.threads_to_use = t;
    }


    pub fn contains (&self, n: u16) -> bool {
        // println!("contains - {} - {}", n, self.low <= n && n <= self.high);
        self.low <= n && n <= self.high
    }

    #[allow(dead_code)]
    pub fn not_contains (&self, n:u16) -> bool {
        // println!("contains - {} - {}", n, self.low <= n && n <= self.high);
        !(self.low <= n && n <= self.high)
    }


    pub fn can_merge (&self, other: &Self) -> bool {
        // (other.low <= self.low && self.high <= other.high)
        // || (self.low <= other.low && other.high <= self.high)
        // || (self.low <= other.low && self.high <= other.high && other.low <= self.high)
        // || (other.low <= self.low && other.high <= self.high && self.low <= other.high)
        // || (self.low == other.high + 1)
        // || (self.high == other.low - 1)

        !(other.high < self.low || self.high < other.low)
        || (self.low == other.high + 1)
        || (self.high == other.low - 1)
    }

    
    pub fn change_range_ports (&mut self, other: &Self) {
        match self.merge_method(other) {
            MergeMethod::OtherContainsSelf => {
                self.low = other.low;
                self.high = other.high;
                self.num = self.high - self.low + 1;
            },
            MergeMethod::SelfContainsOther => {},
            MergeMethod::SelfIsLower => {
                self.high = other.high;
                self.num = self.high - self.low + 1;
            },
            MergeMethod::SelfIsHigher => {
                self.low = other.low;
                self.num = self.high - self.low + 1;
            },
            MergeMethod::SelfLowOtherHigh => {
                self.low = other.low;
                self.num = self.high - self.low + 1;
            },
            MergeMethod::SelfHighOtherLow => {
                self.high = other.high;
                self.num = self.high - self.low + 1;
            },
            MergeMethod::Wrong => {},
        };
    }

    fn merge_method (&self, other: &Self) -> MergeMethod {
        if other.low < self.low && self.high < other.high {
            MergeMethod::OtherContainsSelf
        } else if self.low < other.low && other.high < self.high {
            MergeMethod::SelfContainsOther
        } else if self.low < other.low && self.high < other.high && other.low < self.high {
            MergeMethod::SelfIsLower
        } else if other.low < self.low && other.high < self.high && self.low < other.high {
            MergeMethod::SelfIsHigher
        } else if self.low == other.high + 1 {
            MergeMethod::SelfLowOtherHigh
        } else if self.high == other.low - 1 {
            MergeMethod::SelfHighOtherLow
        } else {
            MergeMethod::Wrong
        }
    }

}


#[cfg(test)]
mod range_tests {

    // cargo t -- --nocapture
    
    use super::RangePorts;

    // NORMAL

    #[test]
    fn first_contains () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(5, 30);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn second_contains () {
        let b = RangePorts::new(10, 20);
        let a = RangePorts::new(5, 30);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn first_part () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(15, 30);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn second_part () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(5, 15);

        assert!(a.can_merge(&b));
    }

    //////////////////////////////////////////////////////////////////////////

    // HAS AN EQUAL NUMBER

    #[test]
    fn first_contains_eq () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(10, 30);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn second_contains_eq () {
        let b = RangePorts::new(10, 20);
        let a = RangePorts::new(5, 20);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn first_part_eq () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(20, 30);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn second_part_eq () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(5, 10);

        assert!(a.can_merge(&b));
    }

    #[test]
    fn first_same () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(10, 20);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn second_same () {
        let b = RangePorts::new(10, 20);
        let a = RangePorts::new(20, 20);

        assert!(a.can_merge(&b));
    }

    //////////////////////////////////////////////////////////////////////////

    // SOULD FAIL
    
    #[test]
    fn first_fail () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(22, 30);

        assert_eq!(a.can_merge(&b), false);
    }
    #[test]
    fn second_fail () {
        let a = RangePorts::new(10, 20);
        let b = RangePorts::new(1, 8);

        assert_eq!(a.can_merge(&b), false);
    }

    //////////////////////////////////////////////////////////////////////////

    // DIFFERENCE OF 1

    #[test]
    fn self_l_other_h () {
        let b = RangePorts::new(30, 50);
        let a = RangePorts::new(20, 29);

        assert!(a.can_merge(&b));
    }
    #[test]
    fn self_h_other_l () {
        let b = RangePorts::new(10, 20);
        let a = RangePorts::new(21, 30);

        assert!(a.can_merge(&b));
    }
}
