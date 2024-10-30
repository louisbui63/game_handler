//! A grid whose number of columns automatically matches the space available.
//! Originally based on the `Grid` widget from iced_aw

use iced::{
    advanced::{
        layout::{Limits, Node},
        mouse::{self, Cursor},
        overlay, renderer,
        widget::{Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event, Element, Event, Length, Point, Rectangle, Size,
};

#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
{
    /// The distribution [`Strategy`](Strategy) of the [`Grid`](Grid).
    strategy: Strategy,
    /// The elements in the [`Grid`](Grid).
    elements: Vec<Element<'a, Message, Theme, Renderer>>,
}

/// The [`Strategy`](Strategy) of how to distribute the columns of the [`Grid`](Grid).
#[derive(Debug)]
pub enum Strategy {
    /// Use `n` columns.
    Columns(usize),
    /// Try to fit as much columns that have a fixed width.
    ColumnWidth(f32),
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Columns(1)
    }
}

impl<'a, Message, Theme, Renderer> Grid<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Creates a [`Grid`](Grid) with ``Strategy::Columns(1)``
    /// Use ``strategy()`` to update the Strategy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`Grid`](Grid) with given elements and ``Strategy::Columns(1)``
    /// Use ``strategy()`` to update the Strategy.
    #[must_use]
    pub fn with_children(children: Vec<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            strategy: Strategy::default(),
            elements: children,
        }
    }

    /// Creates a new empty [`Grid`](Grid).
    /// Elements will be laid out in a specific amount of columns.
    #[must_use]
    pub fn with_columns(columns: usize) -> Self {
        Self {
            strategy: Strategy::Columns(columns),
            elements: Vec::new(),
        }
    }

    /// Creates a new empty [`Grid`](Grid).
    /// Columns will be generated to fill the given space.
    #[must_use]
    pub fn with_column_width(column_width: f32) -> Self {
        Self {
            strategy: Strategy::ColumnWidth(column_width),
            elements: Vec::new(),
        }
    }

    /// Sets the [`Grid`](Grid) Strategy.
    /// Default is ``Strategy::Columns(1)``.
    #[must_use]
    pub fn strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Adds an [`Element`](Element) to the [`Grid`](Grid).
    #[must_use]
    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }

    /// Inserts an [`Element`](Element) into the [`Grid`](Grid).
    pub fn insert<E>(&mut self, element: E)
    where
        E: Into<Element<'a, Message, Theme, Renderer>>,
    {
        self.elements.push(element.into());
    }
}

impl<'a, Message, Theme, Renderer> Default for Grid<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn default() -> Self {
        Self {
            strategy: Strategy::default(),
            elements: Vec::new(),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Grid<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.elements.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements);
    }

    // fn width(&self) -> Length {
    //     Length::Shrink
    // }
    //
    // fn height(&self) -> Length {
    //     Length::Shrink
    // }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        if self.elements.is_empty() {
            return Node::new(Size::ZERO);
        }
        let mut children = tree.children.iter_mut();

        match self.strategy {
            // find out how wide a column is by finding the widest cell in it
            Strategy::Columns(columns) => {
                if columns == 0 {
                    return Node::new(Size::ZERO);
                }

                let mut layouts = Vec::with_capacity(self.elements.len());
                let mut column_widths = Vec::<f32>::with_capacity(columns);

                for (column, element) in (0..columns).cycle().zip(&self.elements) {
                    let layout =
                        element
                            .as_widget()
                            .layout(children.next().unwrap(), renderer, limits);
                    #[allow(clippy::option_if_let_else)]
                    match column_widths.get_mut(column) {
                        Some(column_width) => *column_width = column_width.max(layout.size().width),
                        None => column_widths.insert(column, layout.size().width),
                    }

                    layouts.push(layout);
                }

                let column_aligns =
                    std::iter::once(&0.)
                        .chain(column_widths.iter())
                        .scan(0., |state, width| {
                            *state += width;
                            Some(*state)
                        });
                let grid_width = column_widths.iter().sum();

                build_grid(columns, column_aligns, layouts.into_iter(), grid_width)
            }
            // find number of columns by checking how many can fit
            // Strategy::ColumnWidth(column_width) => {
            //     let column_limits = limits.width(Length::Fixed(column_width));
            //     let max_width = limits.max().width;
            //     let columns = (max_width / column_width).floor() as usize;
            //
            //     let layouts = self
            //         .elements
            //         .iter()
            //         .map(|element| element.as_widget().layout(renderer, &column_limits));
            //     let column_aligns =
            //         std::iter::successors(Some(0.), |width| Some(width + column_width));
            //     #[allow(clippy::cast_precision_loss)] // TODO: possible precision loss
            //     let grid_width = (columns as f32) * column_width;
            //
            //     build_grid(columns, column_aligns, layouts, grid_width)
            // }
            Strategy::ColumnWidth(column_width) => {
                let column_limits = limits.width(Length::Fixed(column_width));
                let max_width = limits.max().width;
                let columns = (max_width / column_width).floor() as usize;

                ////////////
                // let margin = (max_width - columns as f32 * column_width) / columns as f32;
                // if margin isn't an integer, images might not be aligned with the pixel grid
                let margin = (max_width as usize - columns * column_width as usize) / columns;
                ////////////

                let layouts = self.elements.iter().map(|element| {
                    let u = children.next().unwrap();
                    let v = element.as_widget().layout(u, renderer, &column_limits);
                    v
                });

                let column_aligns = std::iter::successors(Some(0.), |width| {
                    Some(width + column_width + margin as f32)
                });
                #[allow(clippy::cast_precision_loss)] // TODO: possible precision loss
                let grid_width = (columns as f32) * (column_width + margin as f32);

                build_grid(columns, column_aligns, layouts, grid_width)
            }
        }
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let children_status = self
            .elements
            .iter_mut()
            .zip(&mut state.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            });

        children_status.fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.elements
            .iter()
            .zip(&state.children)
            .zip(layout.children())
            .map(|((e, state), layout)| {
                e.as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .fold(mouse::Interaction::default(), |interaction, next| {
                interaction.max(next)
            })
    }

    fn operate(
        &self,
        state: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        for ((element, state), layout) in self
            .elements
            .iter()
            .zip(&mut state.children)
            .zip(layout.children())
        {
            element
                .as_widget()
                .operate(state, layout, renderer, operation);
        }
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        for ((element, state), layout) in self
            .elements
            .iter()
            .zip(&state.children)
            .zip(layout.children())
        {
            element
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(&mut self.elements[..], tree, layout, renderer, translation)
    }
}

/// Builds the layout of the [`Grid`](grid).
fn build_grid(
    columns: usize,
    column_aligns: impl Iterator<Item = f32> + Clone,
    layouts: impl ExactSizeIterator<Item = Node>,
    grid_width: f32,
) -> Node {
    let mut nodes = Vec::with_capacity(layouts.len());
    let mut grid_height = 0.;
    let mut row_height = 0.;

    for ((column, column_align), mut node) in (0..columns).zip(column_aligns).cycle().zip(layouts) {
        if column == 0 {
            grid_height += row_height;
            row_height = 0.;
        }

        node = node.move_to(Point::new(column_align, grid_height));
        row_height = row_height.max(node.size().height.ceil());
        nodes.push(node);
    }

    grid_height += row_height;

    Node::with_children(Size::new(grid_width, grid_height), nodes)
}

impl<'a, Message, Theme, Renderer> From<Grid<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
    Message: 'static,
    Theme: 'a,
{
    fn from(grid: Grid<'a, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(grid)
    }
}
