use mathru::analysis::integral::newton_cotes::ClosedFixedIntervalIterator;

#[test]
fn lower_bound()
{
    let lower: f64 = 2.0;
    let upper: f64 = 4.0;
    let num_intervals: u32 = 5;
    let mut cfii: ClosedFixedIntervalIterator<f64> = ClosedFixedIntervalIterator::new(lower, upper, num_intervals);

    assert_eq!(cfii.next(), Some(lower));
}

#[test]
fn upper_bound()
{
    let lower: f64 = 2.0;
    let upper: f64 = 4.0;
    let num_intervals: u32 = 4;
    let mut cfii: ClosedFixedIntervalIterator<f64> = ClosedFixedIntervalIterator::new(lower, upper, num_intervals);

    for _i in 0..num_intervals {
        cfii.next();
    }

    assert_eq!(cfii.next(), Some(upper));
    assert_eq!(cfii.next(), None);
}