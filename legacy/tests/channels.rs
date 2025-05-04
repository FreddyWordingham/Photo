use photo::Channels;

#[test]
fn test_from_num_channels() {
    assert_eq!(Channels::from_num_channels(1), Some(Channels::Grey));
    assert_eq!(Channels::from_num_channels(2), Some(Channels::GreyAlpha));
    assert_eq!(Channels::from_num_channels(3), Some(Channels::RGB));
    assert_eq!(Channels::from_num_channels(4), Some(Channels::RGBA));
    assert_eq!(Channels::from_num_channels(5), None);
}

#[test]
fn test_num_channels() {
    assert_eq!(Channels::Grey.num_channels(), 1);
    assert_eq!(Channels::GreyAlpha.num_channels(), 2);
    assert_eq!(Channels::RGB.num_channels(), 3);
    assert_eq!(Channels::RGBA.num_channels(), 4);
}

#[test]
fn test_has_alpha() {
    assert!(!Channels::Grey.has_alpha());
    assert!(Channels::GreyAlpha.has_alpha());
    assert!(!Channels::RGB.has_alpha());
    assert!(Channels::RGBA.has_alpha());
}

#[test]
fn test_is_greyscale() {
    assert!(Channels::Grey.is_greyscale());
    assert!(Channels::GreyAlpha.is_greyscale());
    assert!(!Channels::RGB.is_greyscale());
    assert!(!Channels::RGBA.is_greyscale());
}

#[test]
fn test_is_colour() {
    assert!(!Channels::Grey.is_colour());
    assert!(!Channels::GreyAlpha.is_colour());
    assert!(Channels::RGB.is_colour());
    assert!(Channels::RGBA.is_colour());
}
