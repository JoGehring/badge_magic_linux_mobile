use libadwaita::gtk::{DropDown, Scale, Stack, StackSwitcher, ToggleButton};
use libadwaita::gtk::StackTransitionType::SlideLeftRight;
use crate::ui::{speed_page, animations_page, effects_page};


/// **Combines the Speed, Effects and Animations widgets to a ```ViewStack```**
///
/// * Calls the ```build_speed_page``` method
/// * Calls the ```build_effects_page``` method
/// * Calls the ```build_animations_page``` method
/// * Create a ```StackSwitcher``` widget
/// * Combine the pages in a ```Stack``` widget
/// * Set the ```Stack``` widget for the ```StackSwitcher```
///
/// # Returns
/// * A ```boxed::Box``` reference to the ```StackSwitcher``` widget
/// * A ```boxed::Box``` reference to the ```Stack``` widget
/// * The leaked input widgets references created in their corresponding methods
pub fn build_view_stack() -> (Box<StackSwitcher>, Box<Stack>, &'static Scale, &'static ToggleButton, &'static ToggleButton, &'static ToggleButton, &'static DropDown) {
    let (speed, scale) = speed_page::build_speed_page();
    let (effects_page, flash, marquee, invert) = effects_page::build_effects_page();
    let (animations_page, drop_down) = animations_page::build_animations_page();
    let stack_switcher = StackSwitcher::builder().build();
    let stack = Stack::builder().width_request(100).build();
    let _page1 = stack.add_titled(speed.as_ref(), Option::<&str>::None, "Speed");
    let _page2 = stack.add_titled(effects_page.as_ref(), Option::<&str>::None, "Effects");
    let _page3 = stack.add_titled(animations_page.as_ref(), Option::<&str>::None, "Mode");
    stack.set_transition_type(SlideLeftRight);
    stack.set_transition_duration(200);
    stack_switcher.set_stack(Option::from(&stack));
    (Box::from(stack_switcher), Box::from(stack), Box::<Scale>::leak(scale), Box::<ToggleButton>::leak(flash), Box::<ToggleButton>::leak(marquee), Box::<ToggleButton>::leak(invert), Box::<DropDown>::leak(drop_down))
}