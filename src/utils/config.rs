use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config =serde_json::from_str(include_str!("../configs/config.json")).unwrap();
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct Config {
    pub default_container_element: Value,
    pub default_icon_pack: String,
    pub default_icon_component: Value,
    pub default_icon_prev: String,
    pub default_icon_next: String,
    pub default_locale: String,
    pub default_dialog_confirm_text: Value,
    pub default_dialog_cancel_text: Value,
    pub default_snackbar_duration: i64,
    pub default_snackbar_position: Value,
    pub default_toast_duration: i64,
    pub default_toast_position: Value,
    pub default_notification_duration: i64,
    pub default_notification_position: Value,
    pub default_tooltip_type: String,
    pub default_tooltip_delay: Value,
    pub default_sidebar_delay: Value,
    pub default_input_autocomplete: String,
    pub default_date_formatter: Value,
    pub default_date_parser: Value,
    pub default_date_creator: Value,
    pub default_time_creator: Value,
    pub default_day_names: Value,
    pub default_month_names: Value,
    pub default_first_day_of_week: Value,
    pub default_unselectable_days_of_week: Value,
    pub default_time_formatter: Value,
    pub default_time_parser: Value,
    pub default_modal_can_cancel: Vec<String>,
    pub default_modal_scroll: Value,
    pub default_datepicker_mobile_native: bool,
    pub default_timepicker_mobile_native: bool,
    pub default_notice_queue: bool,
    pub default_input_has_counter: bool,
    pub default_taginput_has_counter: bool,
    #[serde(rename = "defaultUseHtml5Validation")]
    pub default_use_html5validation: bool,
    pub default_dropdown_mobile_modal: bool,
    pub default_field_label_position: Value,
    pub default_datepicker_years_range: Vec<i64>,
    pub default_datepicker_nearby_month_days: bool,
    pub default_datepicker_nearby_selectable_month_days: bool,
    pub default_datepicker_show_week_number: bool,
    pub default_datepicker_week_number_clickable: bool,
    pub default_datepicker_mobile_modal: bool,
    pub default_trap_focus: bool,
    pub default_auto_focus: bool,
    pub default_button_rounded: bool,
    pub default_switch_rounded: bool,
    pub default_carousel_interval: i64,
    pub default_tabs_expanded: bool,
    pub default_tabs_animated: bool,
    pub default_tabs_type: Value,
    pub default_status_icon: bool,
    pub default_programmatic_promise: bool,
    pub default_link_tags: Vec<String>,
    pub default_image_webp_fallback: Value,
    pub default_image_lazy: bool,
    pub default_image_responsive: bool,
    pub default_image_ratio: Value,
    pub default_image_srcset_formatter: Value,
    pub default_breadcrumb_tag: String,
    pub default_breadcrumb_align: String,
    pub default_breadcrumb_separator: String,
    pub default_breadcrumb_size: String,
    pub custom_icon_packs: Value,
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cfg = &CONFIG;
        assert_eq!(cfg.default_icon_next, "chevron-right");
    }
}
