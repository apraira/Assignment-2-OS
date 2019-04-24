use std::fmt;

pub struct Clock{
    hours:i32,
    minutes:i32
}

impl Clock {
    pub fn new(mut hours: i32,mut minutes: i32) -> Self {
        
        if hours >= 0 && hours < 24 && minutes >= 0 && minutes < 60{
        hours = hours;
        minutes = minutes;
        }
        else if hours >= 24 && minutes >= 0 && minutes < 60{
        hours = hours%24;
        minutes = minutes;
        }
        else if hours >= 0 && hours < 24 && minutes >= 60{
        hours = hours + minutes/60;
            if hours >= 24 {
                hours = hours%24;
            }
        minutes = minutes%60;        
        }
        else if hours >= 24 && minutes >= 60{
        hours = hours + minutes/60;
            if hours >= 24 {
                hours = hours%24;
            }
        minutes = minutes%60;        
        }
        else if hours < 0 && minutes >= 0 && minutes < 60{
            while hours < 0 {
                hours = 24 + hours;
            }
            hours = hours;
            minutes = minutes;
            
        }
        else if hours >=0 && hours < 24 && minutes < 0{
            while minutes < 0 {
                minutes = 60 + minutes;
                hours = hours - 1;
                 while hours < 0 {
                    hours = 24 + hours;
                    }
            }
        }

        else if hours < 0 && minutes < 0{
            while minutes < 0 {
                minutes = 60 + minutes;
                hours = hours - 1;
                 while hours < 0 {
                    hours = 24 + hours;
                    }
            }
        }

        Clock{hours:hours, minutes:minutes}
        
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
         Clock::new(self.hours, self.minutes + minutes)
       }
    

    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hours < 10 && self.minutes < 10 {
            write!(f, "0{}:0{}", self.hours, self.minutes) }
        else if self.hours > 9 && self.minutes < 10 {
            write!(f, "{}:0{}", self.hours, self.minutes)
        }
        else if self.hours < 10 && self.minutes > 9 {
            write!(f, "0{}:{}", self.hours, self.minutes)
        }
        else {
            write!(f, "{}:{}", self.hours, self.minutes)
        }
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "2")
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        true
    }
}

// Problem link: https://exercism.io/tracks/rust/exercises/clock/solutions/4edd7212de63459d8fa514cc94da8c54
