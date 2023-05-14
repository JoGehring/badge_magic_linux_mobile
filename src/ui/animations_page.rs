use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, DropDown,Widget};
use libadwaita::prelude::{BoxExt, WidgetExt};

pub fn build_animations_page() -> (impl IsA<Widget>, DropDown) {
    let animations_page = Box::builder()
        .css_classes(["entry_box", "animations"])
        .build();
    let drop_down = DropDown::from_strings(["Scroll Left", "Scroll Right", "Scroll Up", "Scroll Down", "Still Centered", "Animation", "Drop Down", "Curtain", "Laser"].as_ref());
    drop_down.set_hexpand(true);
    animations_page.append(&drop_down);
    (animations_page, drop_down)
}