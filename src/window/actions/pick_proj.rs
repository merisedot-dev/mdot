use gtk::{FileDialog, gio::Cancellable};

use crate::{utils::MDotAction, window::Window};

pub struct PickProjectAction;

impl MDotAction for PickProjectAction {
    type InnerCallerType = Window;

    fn name(&self) -> &'static str {
        "win.pick_proj"
    }

    fn handle_activate(
        &self,
        caller: &Self::InnerCallerType,
        _: &str,
        _: Option<&gtk::glib::Variant>,
    ) {
        // TODO define filters
        // building Dialog
        let dialog = FileDialog::builder()
            .title("Choose project")
            .accept_label("Open")
            .build();

        // TODO runnning dialog
        dialog.open(Some(caller), Cancellable::NONE, |_| {});
    }
}
