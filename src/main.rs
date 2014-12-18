mod tests {
    extern crate time;
    extern crate gaeta;

    use self::gaeta::{TimeContext,TestTimer,GetTimestamp};
    use std::io::timer;
    use std::time::Duration;

    // Implementation of a `SystemTimer`. This struct is needed to provide current system
    // time.
    //
    // Other implementations of `GetTimestamp` trait can provide different timestamp values,
    // depending of what's needed. Gaeta internally uses TestTimer that returns deterministic
    // values in order to perform unit testing.
    struct SystemTimer;

    impl SystemTimer {
        fn new() -> SystemTimer { SystemTimer }
    }

    // Trait implementation.
    impl GetTimestamp for SystemTimer {
        fn get_timestamp(&self) -> u64 {
            // Converts nanosecond to a millisecond.

            time::precise_time_ns() / 1_000_000
        }
    }

    // A fuzzy comparison helper function. It is parametrized using traits:
    //
    // - PartialOrd: to allow types that implement >= and <=,
    // - Add:        to allow types that implement +,
    // - Sub:        to allow types that implement -,
    // - Copy:       to allow types that can be instantiated by copying. This is needed since
    //               `fuzzy_cmp` uses parameters `a`, `b` and `tolerance` in multiple instances.
    fn fuzzy_cmp<T: PartialOrd + Add<T,T> + Sub<T,T> + Copy>(a: T, b: T, tolerance: T) -> bool {
        a >= b - tolerance && a <= b + tolerance
    }

    #[test]
    fn test_uninitialized() {
        let tc = TimeContext::new(TestTimer::new());
        assert!(tc.get_remaining_time() == 0);
    }

    #[test]
    fn test_0_system() {
        let mut tc = TimeContext::new(SystemTimer::new());

        for i in range(0, 90) {
            tc.update_eta(i, 100);
            timer::sleep(Duration::milliseconds(10));
        }

        assert!(fuzzy_cmp(tc.get_remaining_time(), 110, 10));
    }

    #[test]
    fn test_0() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(0);
        tc.update_eta(0, 500);

        tc.get_timefunc_mut().set_timestamp(750);
        tc.update_eta(1, 500);
        assert!(fuzzy_cmp(tc.calc_speed_per_unit(), 0.00026, 0.00001));

        tc.get_timefunc_mut().set_timestamp(1325);
        tc.update_eta(2, 500);
        assert!(fuzzy_cmp(tc.calc_speed_per_unit(), 0.000284, 0.000005));

        tc.get_timefunc_mut().set_timestamp(2222);
        tc.update_eta(3, 500);
        assert!(fuzzy_cmp(tc.calc_speed_per_unit(), 0.00028, 0.00001));

        tc.get_timefunc_mut().set_timestamp(2323);
        tc.update_eta(4, 500);
        assert!(fuzzy_cmp(tc.calc_speed_per_unit(), 0.000296, 0.000005));

        tc.get_timefunc_mut().set_timestamp(5000);
        tc.update_eta(5, 500);
        assert!(fuzzy_cmp(tc.calc_speed_per_unit(), 0.000277, 0.000005));
    }

    #[test]
    fn test_2() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(0);
        tc.update_eta(0, 500);

        tc.get_timefunc_mut().set_timestamp(750);
        tc.update_eta(1, 500);

        tc.get_timefunc_mut().set_timestamp(1325);
        tc.update_eta(2, 500);

        tc.get_timefunc_mut().set_timestamp(2222);
        tc.update_eta(3, 500);

        tc.get_timefunc_mut().set_timestamp(2323);
        tc.update_eta(4, 500);

        tc.get_timefunc_mut().set_timestamp(5000);
        tc.update_eta(5, 500);

        assert!(tc.get_remaining_time() / 1000 == 357);
    }

    #[test]
    fn test_3() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(0);
        tc.update_eta(0, 100);

        tc.get_timefunc_mut().set_timestamp(1000);
        tc.update_eta(1, 100);

        tc.get_timefunc_mut().set_timestamp(2000);
        tc.update_eta(2, 100);

        tc.get_timefunc_mut().set_timestamp(3000);
        tc.update_eta(3, 100);

        tc.get_timefunc_mut().set_timestamp(4000);
        tc.update_eta(4, 100);

        assert!(tc.get_remaining_time() / 1000 == 96);
    }

    #[test]
    fn test_4() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(0);
        tc.update_eta(50, 100);

        tc.get_timefunc_mut().set_timestamp(1000);
        tc.update_eta(51, 100);

        tc.get_timefunc_mut().set_timestamp(2000);
        tc.update_eta(52, 100);

        tc.get_timefunc_mut().set_timestamp(3000);
        tc.update_eta(53, 100);

        tc.get_timefunc_mut().set_timestamp(4000);
        tc.update_eta(54, 100);

        let remaining_ms = tc.get_remaining_time();
        assert!(remaining_ms / 1000 == 96);
    }

    #[test]
    fn test_5() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(0);
        tc.update_eta(51, 100);

        tc.get_timefunc_mut().set_timestamp(1000);
        tc.update_eta(52, 100);

        tc.get_timefunc_mut().set_timestamp(2000);
        tc.update_eta(53, 100);

        tc.get_timefunc_mut().set_timestamp(3000);
        tc.update_eta(54, 100);

        tc.get_timefunc_mut().set_timestamp(4000);
        tc.update_eta(55, 100);

        let remaining_ms = tc.get_remaining_time();
        assert!(remaining_ms / 1000 == 96);
    }

    #[test]
    fn test_6() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(1000);
        tc.update_eta(0, 100);

        tc.get_timefunc_mut().set_timestamp(2000);
        tc.update_eta(1, 100);

        tc.get_timefunc_mut().set_timestamp(3000);
        tc.update_eta(2, 100);

        tc.get_timefunc_mut().set_timestamp(4000);
        tc.update_eta(3, 100);

        tc.get_timefunc_mut().set_timestamp(5000);
        tc.update_eta(4, 100);

        assert!(tc.get_remaining_time() / 1000 == 96);
    }

    #[test]
    fn test_7() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(1000);
        tc.update_eta(50, 100);

        tc.get_timefunc_mut().set_timestamp(2000);
        tc.update_eta(51, 100);

        tc.get_timefunc_mut().set_timestamp(3000);
        tc.update_eta(52, 100);

        tc.get_timefunc_mut().set_timestamp(4000);
        tc.update_eta(53, 100);

        tc.get_timefunc_mut().set_timestamp(5000);
        tc.update_eta(54, 100);

        assert!(tc.get_remaining_time() / 1000 == 96);
    }

    #[test]
    fn test_8() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(0);
        tc.update_eta(0, 100);

        tc.get_timefunc_mut().set_timestamp(30);
        tc.update_eta(17, 100);

        tc.get_timefunc_mut().set_timestamp(70);
        tc.update_eta(35, 100);

        tc.get_timefunc_mut().set_timestamp(110);
        tc.update_eta(60, 100);

        tc.get_timefunc_mut().set_timestamp(150);
        tc.update_eta(72, 100);

        tc.get_timefunc_mut().set_timestamp(190);
        tc.update_eta(99, 100);

        assert!(tc.get_remaining_time() == 1);
    }

    #[test]
    fn test_9() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(1000);
        tc.update_eta(0, 100);

        tc.get_timefunc_mut().set_timestamp(1030);
        tc.update_eta(17, 100);

        tc.get_timefunc_mut().set_timestamp(1070);
        tc.update_eta(35, 100);

        tc.get_timefunc_mut().set_timestamp(1110);
        tc.update_eta(60, 100);

        tc.get_timefunc_mut().set_timestamp(1150);
        tc.update_eta(72, 100);

        tc.get_timefunc_mut().set_timestamp(1190);
        tc.update_eta(99, 100);

        assert!(tc.get_remaining_time() == 1);
    }

    #[test]
    fn test_10() {
        let mut tc = TimeContext::new(TestTimer::new());

        tc.get_timefunc_mut().set_timestamp(1000000);
        tc.update_eta(0, 100);

        tc.get_timefunc_mut().set_timestamp(1030000);
        tc.update_eta(17, 100);

        tc.get_timefunc_mut().set_timestamp(1070000);
        tc.update_eta(35, 100);

        tc.get_timefunc_mut().set_timestamp(1110000);
        tc.update_eta(60, 100);

        tc.get_timefunc_mut().set_timestamp(1150000);
        tc.update_eta(72, 100);

        tc.get_timefunc_mut().set_timestamp(1190000);
        tc.update_eta(99, 100);

        assert!(tc.get_remaining_time() == 1913);
    }
}

