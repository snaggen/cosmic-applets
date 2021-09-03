use cascade::cascade;
use gtk4::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

use crate::deref_cell::DerefCell;
use crate::mpris::MprisControls;
use crate::popover_container::PopoverContainer;

#[derive(Default)]
pub struct TimeButtonInner {
    calendar: DerefCell<gtk4::Calendar>,
    button: DerefCell<gtk4::ToggleButton>,
}

#[glib::object_subclass]
impl ObjectSubclass for TimeButtonInner {
    const NAME: &'static str = "S76TimeButton";
    type ParentType = gtk4::Widget;
    type Type = TimeButton;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk4::BinLayout>();
    }
}

impl ObjectImpl for TimeButtonInner {
    fn constructed(&self, obj: &TimeButton) {
        let calendar = cascade! {
            gtk4::Calendar::new();
        };

        let button = cascade! {
            gtk4::ToggleButton::new();
            ..set_has_frame(false);
        };

        cascade! {
            PopoverContainer::new(&button);
            ..set_parent(obj);
            ..popover().set_child(Some(&cascade! {
                gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                ..append(&MprisControls::new());
                ..append(&calendar);
            }));
            ..popover().connect_show(clone!(@strong obj => move |_| obj.opening()));
            ..popover().bind_property("visible", &button, "active").flags(glib::BindingFlags::BIDIRECTIONAL).build();
        };

        self.calendar.set(calendar);
        self.button.set(button);

        // TODO: better way to do this?
        glib::timeout_add_seconds_local(
            1,
            clone!(@weak obj => @default-return glib::Continue(false), move || {
                obj.update_time();
                glib::Continue(true)
            }),
        );
        obj.update_time();
    }

    fn dispose(&self, _obj: &TimeButton) {
        self.button.unparent();
    }
}

impl WidgetImpl for TimeButtonInner {}

glib::wrapper! {
    pub struct TimeButton(ObjectSubclass<TimeButtonInner>)
        @extends gtk4::Widget;
}

impl TimeButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    fn inner(&self) -> &TimeButtonInner {
        TimeButtonInner::from_instance(self)
    }

    fn opening(&self) {
        let date = glib::DateTime::new_now(&glib::TimeZone::new_local()).unwrap();
        self.inner().calendar.clear_marks();
        self.inner().calendar.select_day(&date);
    }

    fn update_time(&self) {
        // TODO: Locale-based formatting?
        let time = chrono::Local::now();
        self.inner()
            .button
            .set_label(&time.format("%b %-d %-I:%M %p").to_string());
        // time.format("%B %-d %Y")
    }
}
