use crate::ui::component::Component;

pub trait BootloaderFrame{
    fn repaint(&mut self);
    fn messages(&mut self, msg: <Self as Component>::Msg) -> Option<u32> where Self: Component;
}