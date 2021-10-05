use super::*;

#[test]
fn basic_test_for_type_positiontype() {
    let (width, height, temp) = (800, 600, 100);
    assert_eq!(
        PositionType::TopLeftBottomRight.to_rect(width, height, 0, 0, 0, 0),
        (0, 0, width, height)
    );
    assert_eq!(
        PositionType::TopLeftWidthHeight.to_rect(width, height, 0, 0, temp, temp),
        (0, 0, temp, temp)
    );
}

#[test]
#[ignore]
fn full_test_for_type_positiontype() {
    basic_test_for_type_positiontype();
    let (width, height, temp) = (800, 600, 100);
    assert_eq!(
        PositionType::TopRightWidthHeight.to_rect(width, height, 0, 0, temp, temp),
        (width - temp, 0, width, temp)
    );
    assert_eq!(
        PositionType::TopLeftBottomWidth.to_rect(width, height, 0, 0, 0, temp),
        (0, 0, temp, height)
    );
    assert_eq!(
        PositionType::TopBottomRightWidth.to_rect(width, height, 0, 0, 0, temp),
        (width - temp, 0, width, height)
    );
    assert_eq!(
        PositionType::TopLeftRightHeight.to_rect(width, height, 0, 0, 0, temp),
        (0, 0, width, temp)
    );
    assert_eq!(
        PositionType::LeftBottomRightHeight.to_rect(width, height, 0, 0, 0, temp),
        (0, height - temp, width, height)
    );
}
