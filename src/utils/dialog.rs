use crate::utils::MDotActable;

pub trait MDotDialog: MDotActable {
    fn name(&self) -> &'static str;
    async fn handle_dialog(&self, caller: Self::InnerCallerType);
}
