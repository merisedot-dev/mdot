use crate::utils::MDotActable;

pub trait MDotDialog: MDotActable {
    async fn handle_dialog(&self, caller: Self::InnerCallerType);
}
