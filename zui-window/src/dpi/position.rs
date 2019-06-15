/// A position represented in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalPosition {
    /// The X offset in pixels.
    pub x: f64,
    /// The Y offset in pixels.
    pub y: f64,
}

impl From<(f64, f64)> for PhysicalPosition {
    #[inline]
    fn from((w, h): (f64, f64)) -> Self {
        Self::new(w, h)
    }
}

impl From<PhysicalPosition> for (f64, f64) {
    #[inline]
    fn from(PhysicalPosition { x, y }: PhysicalPosition) -> Self {
        (x, y)
    }
}

impl PhysicalPosition {
    /// Creates a new position of `x` and `y`.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        PhysicalPosition { x, y }
    }

    /// Scales a `LogicalPosition` to a `PhysicalPosition` by `dpi_factor`.
    #[inline]
    pub fn from_logical(position: LogicalPosition, dpi_factor: f64) -> Self {
        position.to_physical(dpi_factor)
    }

    /// Scales `self` to a `LogicalPosition` by `dpi_factor`.
    #[inline]
    pub fn to_logical(&self, dpi_factor: f64) -> LogicalPosition {
        (self.x / dpi_factor, self.y / dpi_factor).into()
    }
}

/// A position represented in logical pixels, or "points".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogicalPosition {
    /// The X offset in points.
    pub x: f64,
    /// The Y offset in points.
    pub y: f64,
}

impl From<(f64, f64)> for LogicalPosition {
    #[inline]
    fn from((w, h): (f64, f64)) -> Self {
        Self::new(w, h)
    }
}

impl From<(i32, i32)> for LogicalPosition {
    #[inline]
    fn from((w, h): (i32, i32)) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl From<LogicalPosition> for (f64, f64) {
    #[inline]
    fn from(LogicalPosition { x, y }: LogicalPosition) -> Self {
        (x, y)
    }
}

impl From<LogicalPosition> for (i32, i32) {
    #[inline]
    fn from(LogicalPosition { x, y }: LogicalPosition) -> Self {
        (x.round() as _, y.round() as _)
    }
}

impl LogicalPosition {
    /// Creates a new position of `x` and `y`.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        LogicalPosition { x, y }
    }

    /// Scales a `LogicalPosition` to a `PhysicalPosition` by `dpi_factor`.
    #[inline]
    pub fn from_physical(position: PhysicalPosition, dpi_factor: f64) -> Self {
        position.to_logical(dpi_factor)
    }

    /// Scales `self` to a `LogicalPosition` by `dpi_factor`.
    #[inline]
    pub fn to_physical(&self, dpi_factor: f64) -> PhysicalPosition {
        (self.x * dpi_factor, self.y * dpi_factor).into()
    }
}
