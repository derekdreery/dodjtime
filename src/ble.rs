use defmt::unwrap;

async fn advertise(sd: &SoftDevice) {
    let mut config = AdvertiseConfig::default();
    config.timeout = Some(10);
    #[rustfmt::skip]
    let adv_data = &[
        0x02, 0x01, nrf_softdevice::raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8,
        0x03, 0x03, 0x09, 0x18,
        0x09, 0x09, b'D', b'o', b'd', b'j', b't', b'i', b'm', b'e',
    ];
    #[rustfmt::skip]
    let scan_data = &[
        0x03, 0x03, 0x09, 0x18,
    ];
    let conn = unwrap!(
        advertise(
            softdev,
            ScannableUndirected {
                adv_data,
                scan_data
            },
            &config,
        )
        .await
    );
}
