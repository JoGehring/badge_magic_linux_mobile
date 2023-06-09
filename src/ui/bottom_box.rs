
use libadwaita::gtk::{Box, Button, CenterBox, Image, Label, Orientation};
use libadwaita::prelude::{BoxExt, ButtonExt};
use std::boxed;

/// **Builds the widget for the Save and Transfer button at the bottom of the screen**
///
/// * Creates a ```gtk::Box``` widget
/// * Creates two ```Button``` widgets
/// * Sets the button ```Label``` and icons accordingly
/// * Combine everything together
///
/// # Returns
/// * A ```boxed::Box``` reference to the combined ```gtk::Box``` widget
/// * A ```boxed::Box``` reference to the Save ```Button``` widget for easy access
/// * A ```boxed::Box``` reference to the Transfer ```Button``` widget for easy access
pub fn build_bottom_box() -> (boxed::Box<CenterBox>, boxed::Box<Button>, boxed::Box<Button>) {
    let container = Box::builder()

        
        .spacing(10)
        .orientation(Orientation::Horizontal)
        .build();
    let bottom_box = CenterBox::builder().css_classes(["bottom_box"]).build();
    let save_button_label_box = Box::new(Orientation::Horizontal, 5);
    save_button_label_box.prepend(&Image::from_icon_name("document-save"));
    let save_button = Button::builder().build();
    save_button_label_box.append(&Label::new(Some("Save")));
    save_button.set_child(Some(&save_button_label_box));
    let transfer_button_label_box = Box::new(Orientation::Horizontal, 5);
    transfer_button_label_box.prepend(&Image::from_icon_name("go-up"));
    transfer_button_label_box.append(&Label::new(Some("Transfer")));

    let transfer_button = Button::builder().build();
    transfer_button.set_child(Some(&transfer_button_label_box));
    container.append(&save_button);
    container.append(&transfer_button);
    bottom_box.set_center_widget(Some(&container));
    (boxed::Box::from(bottom_box), boxed::Box::from(save_button), boxed::Box::from(transfer_button))
}