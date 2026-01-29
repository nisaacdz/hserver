use crate::settings::ReservationSettings;
use bigdecimal::{BigDecimal, FromPrimitive};

/// Calculate the cost for a single reservation item
/// Adds room class price and amenities total
pub fn calculate_item_cost(
    room_class_price: BigDecimal,
    amenities_total: BigDecimal,
    _: &ReservationSettings,
) -> BigDecimal {
    room_class_price + amenities_total
}

/// Calculate discount amount based on total and discount rate
pub fn calculate_reservation_discount(
    tot: BigDecimal,
    settings: &ReservationSettings,
) -> BigDecimal {
    tot * BigDecimal::from_f64(settings.discount_rate).unwrap_or_default()
}

/// Calculate VAT amount based on total and VAT rate
pub fn calculate_reservation_vat(tot: BigDecimal, settings: &ReservationSettings) -> BigDecimal {
    tot * BigDecimal::from_f64(settings.vat_rate).unwrap_or_default()
}

/// Calculate final payable amount: total + VAT - discount
pub fn calculate_reservation_final(
    tot: BigDecimal,
    discount: BigDecimal,
    vat: BigDecimal,
) -> BigDecimal {
    tot + vat - discount
}
