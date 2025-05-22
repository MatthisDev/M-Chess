pub mod ws;
pub use ws::{WsContext, WsProvider};

#[macro_export]
macro_rules! ws_context {
    () => {
        use_context::<$crate::ws::WsContext>()
    };
}
