use types::*;

pub enum GuiManagerMessage {
   RedrawEvent(GuiWidgetID),
   SetCursorEvent(GuiWidgetID, &'static str)
}

