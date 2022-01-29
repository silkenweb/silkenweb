use discard::DiscardOnDrop;
use futures_signals::CancelableFutureHandle;

use crate::hydration::node::HydrationText;

pub struct Text {
    pub(super) hydro_text: HydrationText,
}

impl Text {
    pub fn eval_dom_text(&self) -> web_sys::Text {
        self.hydro_text.eval_dom_text()
    }

    pub fn take_futures(&mut self) -> Vec<DiscardOnDrop<CancelableFutureHandle>> {
        Vec::new()
    }
}

pub fn text(text: &str) -> Text {
    Text {
        hydro_text: HydrationText::new(text),
    }
}
