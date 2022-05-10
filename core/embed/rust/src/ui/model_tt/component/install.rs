use crate::ui::{
    component::{Child, Component, Event, EventCtx},
    geometry::{Grid, Rect},
};


pub enum InstallMsg<T, I, M, L, R>  {
    Label(T),
    Icon(I),
    Message(M),
    Left(L),
    Right(R),
}

pub struct Install<T, I, M, L, R> {
    label: Child<T>,
    icon: Child<I>,
    message: Child<M>,
    left: Child<L>,
    right: Child<R>,
}


impl<T, I, M, L, R> Install<T, I, M, L, R>
    where
        T: Component,
        I: Component,
        M: Component,
        L: Component,
        R: Component,
{
    pub fn new(label: T, icon: I, message: M, left: L, right: R) -> Self {
        Self {
            label: Child::new(label),
            icon: Child::new(icon),
            message: Child::new(message),
            left: Child::new(left),
            right: Child::new(right),
        }
    }

    pub fn inner(&self) -> &M {
        self.message.inner()
    }
}

impl<T, I, M, L, R> Component for Install<T, I, M, L, R>
    where
        T: Component,
        I: Component,
        M: Component,
        L: Component,
        R: Component,
{

    type Msg = InstallMsg<T::Msg, I::Msg, M::Msg, L::Msg, R::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        let layout = InstallLayout::middle(bounds);
        self.label.place(layout.label);
        self.icon.place(layout.icon);
        self.message.place(layout.message);
        self.left.place(layout.left);
        self.right.place(layout.right);
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.message
            .event(ctx, event)
            .map(Self::Msg::Message)
            .or_else(|| self.left.event(ctx, event).map(Self::Msg::Left))
            .or_else(|| self.right.event(ctx, event).map(Self::Msg::Right))
    }

    fn paint(&mut self) {
        self.label.paint();
        self.icon.paint();
        self.message.paint();
        self.left.paint();
        self.right.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.label.bounds(sink);
        self.icon.bounds(sink);
        self.message.bounds(sink);
        self.left.bounds(sink);
        self.right.bounds(sink);
    }
}

pub struct InstallLayout {
    pub label: Rect,
    pub icon: Rect,
    pub message: Rect,
    pub left: Rect,
    pub right: Rect
}

impl InstallLayout {
    pub fn middle(area: Rect) -> Self {
        let grid = Grid::new(area, 10, 6);
        Self {
            label: Rect::new(
                grid.row_col(0, 0).top_left(),
                grid.row_col(0, 5).bottom_right(),
            ),
            icon: Rect::new(
                grid.row_col(1, 0).top_left(),
                grid.row_col(7, 1).bottom_right(),
            ),
            message: Rect::new(
                grid.row_col(1, 2).top_left(),
                grid.row_col(7, 5).bottom_right(),
            ),
            left: Rect::new(
                grid.row_col(8, 0).top_left(),
                grid.row_col(9, 2).bottom_right(),
            ),
            right: Rect::new(
                grid.row_col(8, 3).top_left(),
                grid.row_col(9, 5).bottom_right(),
            ),
        }
    }
}

