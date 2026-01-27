use app::settings::ReservationSettings;
use bigdecimal::BigDecimal;
use num_traits::FromPrimitive;

pub fn calculate_item_cost(
    room_class_price: BigDecimal,
    amenities_total: BigDecimal,
    _: &ReservationSettings,
) -> BigDecimal {
    room_class_price + amenities_total
}

pub fn calculate_reservation_discount(
    tot: BigDecimal,
    settings: &ReservationSettings,
) -> BigDecimal {
    tot * BigDecimal::from_f64(settings.discount_rate).unwrap_or_default()
}

pub fn calculate_reservation_vat(tot: BigDecimal, settings: &ReservationSettings) -> BigDecimal {
    tot * BigDecimal::from_f64(settings.vat_rate).unwrap_or_default()
}

pub fn calculate_reservation_final(
    tot: BigDecimal,
    discount: BigDecimal,
    vat: BigDecimal,
) -> BigDecimal {
    tot + vat - discount
}
