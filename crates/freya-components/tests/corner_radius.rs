use freya::prelude::*;
use freya_components::theming::macros::ResolvablePreference;
use freya_testing::prelude::*;

#[test]
fn radius_shorthands_keep_the_default_sizes() {
    const SIZES: [CornerRadiusSize; 7] = [
        CornerRadiusSize::xs(),
        CornerRadiusSize::sm(),
        CornerRadiusSize::md(),
        CornerRadiusSize::lg(),
        CornerRadiusSize::xl(),
        CornerRadiusSize::xxl(),
        CornerRadiusSize::xxxl(),
    ];
    let base = CornerRadiusThemePreference::default().base_radius;

    for (size, pixels) in SIZES.into_iter().zip([4., 6., 8., 10., 12., 14., 20.]) {
        assert_eq!(size.resolve(base), CornerRadius::new_all(pixels));
        assert_eq!(size.resolve(base * 2.), CornerRadius::new_all(pixels * 2.));
    }
}

#[test]
fn relative_radii_follow_the_theme_base_and_fixed_radii_do_not() {
    let mut theme = Theme::new("custom", LIGHT_COLORS);
    let medium = Preference::<CornerRadiusSize>::CornerRadius(CornerRadiusRole::Md);
    let large = Preference::<CornerRadiusSize>::CornerRadius(CornerRadiusRole::Xl);
    assert_eq!(medium.resolve_with_theme(&theme), CornerRadius::new_all(8.));
    assert_eq!(large.resolve_with_theme(&theme), CornerRadius::new_all(12.));

    let fixed = CornerRadius::new(1., 2., 3., 4.).with_smoothing(0.5);
    theme.set(
        "corner_radius",
        CornerRadiusThemePreference {
            base_radius: 16.,
            xl: CornerRadiusSize::Fixed(fixed),
            ..CornerRadiusThemePreference::default()
        },
    );
    assert_eq!(
        medium.resolve_with_theme(&theme),
        CornerRadius::new_all(16.)
    );
    assert_eq!(large.resolve_with_theme(&theme), fixed);
    assert_eq!(
        Preference::Specific(CornerRadiusSize::Fixed(fixed)).resolve(&theme.colors),
        fixed
    );
}

#[test]
fn changing_the_scale_updates_components_and_elements_but_not_fixed_overrides() {
    fn app() -> impl IntoElement {
        rect()
            .child(Button::new().child("Themed"))
            .child(
                Button::new()
                    .corner_radius(CornerRadiusSize::Relative(0.5))
                    .child("Relative override"),
            )
            .child(Button::new().corner_radius(3.).child("Fixed"))
            .child(
                rect()
                    .width(Size::px(40.))
                    .height(Size::px(40.))
                    .theme_corner_radius(CornerRadiusSize::Relative(1.5)),
            )
    }

    let (mut runner, mut theme) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(light_theme())),
        1.,
    );

    let radii = |runner: &TestingRunner| {
        runner.find_many(|_, element| {
            Rect::try_downcast(element)
                .map(|rect| rect.style.corner_radius.top_left())
                .filter(|radius| *radius != 0.)
        })
    };

    assert_eq!(radii(&runner), [8., 4., 3., 12.]);

    let mut updated = runner.run_in(light_theme);
    updated.set(
        "corner_radius",
        CornerRadiusThemePreference {
            base_radius: 16.,
            md: CornerRadiusSize::Relative(0.75),
            ..CornerRadiusThemePreference::default()
        },
    );
    theme.set_if_modified(updated);
    runner.sync_and_update();

    assert_eq!(radii(&runner), [12., 8., 3., 24.]);

    let mut updated = runner.run_in(light_theme);
    updated.set(
        "corner_radius",
        CornerRadiusThemePreference {
            base_radius: 16.,
            md: CornerRadiusSize::Fixed(CornerRadius::new_all(5.)),
            ..CornerRadiusThemePreference::default()
        },
    );
    theme.set_if_modified(updated);
    runner.sync_and_update();

    assert_eq!(radii(&runner), [5., 8., 3., 24.]);
}
