use stylist::GlobalStyle;

pub(crate) fn initialize() -> Result<(), stylist::Error> {
    let _ = GlobalStyle::new(
        r#"
        --tw-gradient-to-position: 100%;
        --tw-gradient-via-position: 50%;
        --tw-gradient-from-position: 0%;
        "#,
    )?;

    return Ok(());
}
