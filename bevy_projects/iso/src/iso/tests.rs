use super::*;

#[test]
fn line_to() {
    // Same start and end
    let start = Iso::ZERO;
    assert_eq!(start.line_to(start).collect::<Vec<_>>(), vec![start]);

    // Known strait
    let end = Iso::new(5, 0, 0);
    assert_eq!(
        start.line_to(end).collect::<Vec<_>>(),
        vec![
            Iso::new(0, 0, 0),
            Iso::new(1, 0, 0),
            Iso::new(2, 0, 0),
            Iso::new(3, 0, 0),
            Iso::new(4, 0, 0),
            Iso::new(5, 0, 0),
        ]
    );
}

#[test]
fn ring() {
    // Zero
    let center = Iso::ZERO;
    assert_eq!(center.ring(0).collect::<Vec<_>>(), vec![center]);

    // Only neighbors
    let neighbors = center.all_neighbors();
    // Because the order might be different
    for neighbor in neighbors {
        assert!(center.ring(1).collect::<Vec<_>>().contains(&neighbor));
    }

    // Center is not Zero
    let target = Iso::new(14, 7, 0);
    let expecteds = center.ring(10).map(|h| h + target).collect::<Vec<_>>();
    // Because the order might be different
    for expected in expecteds {
        assert!(target.ring(10).collect::<Vec<_>>().contains(&expected));
    }

    // Every ring between 0 and 1000 range is between 0.5 from the ideal
    for range in 0..1000 {
        let result = center.ring(range).collect::<HashSet<Iso>>();
        for point in &result {
            // Calculate Euclidean distance
            let distance =
                (((point.x - center.x).pow(2) + (point.y - center.y).pow(2)) as f64).sqrt();
            assert!(
                (distance - range as f64).abs() < 0.5,
                "Point {:?} is not at the correct distance from center {:?}",
                point,
                center
            );
        }
    }
}
