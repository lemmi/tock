use kernel::hil::time::{self, Alarm, Frequency};

pub struct Accelerate<'a, A: Alarm + 'a> {
    alarm: &'a A,
}

impl<'a, A: Alarm> Accelerate<'a, A> {
    pub fn new(alarm: &'a A) -> Accelerate<'a, A> {
        Accelerate { alarm: alarm }
    }

    pub fn start(&self) {
        self.alarm
            .set_alarm(self.alarm.now().wrapping_add(<A::Frequency>::frequency()))
    }
}

impl<'a, A: Alarm> time::Client for Accelerate<'a, A> {
    fn fired(&self) {
        self.start();
        debug!("Hello world");
    }
}
