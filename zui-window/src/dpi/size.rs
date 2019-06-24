/// A size represented in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicalSize {
    /// The width in pixels.
    pub width: f64,
    /// The height in pixels.
    pub height: f64,
}

impl From<(f64, f64)> for PhysicalSize {
    #[inline]
    fn from((w, h): (f64, f64)) -> Self {
        Self::new(w, h)
    }
}

impl From<(i32, i32)> for PhysicalSize {
    #[inline]
    fn from((w, h): (i32, i32)) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl From<PhysicalSize> for (f64, f64) {
    #[inline]
    fn from(PhysicalSize { width, height }: PhysicalSize) -> Self {
        (width, height)
    }
}

impl From<PhysicalSize> for (i32, i32) {
    #[inline]
    fn from(PhysicalSize { width, height }: PhysicalSize) -> Self {
        (width.round() as _, height.round() as _)
    }
}

impl PhysicalSize {
    /// Creates a new size of `width` and `height`.
    #[inline]
    pub const fn new(width: f64, height: f64) -> Self {
        PhysicalSize { width, height }
    }

    /// Scales a `LogicalSize` to a `PhysicalSize` by `dpi_factor`.
    #[inline]
    pub fn from_logical(size: LogicalSize, dpi_factor: f64) -> Self {
        size.to_physical(dpi_factor)
    }

    /// Scales `self` to a `LogicalSize` by `dpi_factor`.
    #[inline]
    pub fn to_logical(&self, dpi_factor: f64) -> LogicalSize {
        (self.width / dpi_factor, self.height / dpi_factor).into()
    }
}

/// A size represented in logical pixels, or "points".
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogicalSize {
    /// The width in points.
    pub width: f64,
    /// The height in points.
    pub height: f64,
}

impl From<(f64, f64)> for LogicalSize {
    #[inline]
    fn from((w, h): (f64, f64)) -> Self {
        Self::new(w, h)
    }
}

impl From<(i32, i32)> for LogicalSize {
    #[inline]
    fn from((w, h): (i32, i32)) -> Self {
        Self::new(w.into(), h.into())
    }
}

impl From<LogicalSize> for (f64, f64) {
    #[inline]
    fn from(LogicalSize { width, height }: LogicalSize) -> Self {
        (width, height)
    }
}

impl From<LogicalSize> for (i32, i32) {
    #[inline]
    fn from(LogicalSize { width, height }: LogicalSize) -> Self {
        (width.round() as _, height.round() as _)
    }
}

impl LogicalSize {
    /// Creates a new size of `width` and `height`.
    #[inline]
    pub const fn new(width: f64, height: f64) -> Self {
        LogicalSize { width, height }
    }

    /// Scales a `LogicalSize` to a `PhysicalSize` by `dpi_factor`.
    #[inline]
    pub fn from_physical(size: PhysicalSize, dpi_factor: f64) -> Self {
        size.to_logical(dpi_factor)
    }

    /// Scales `self` to a `LogicalSize` by `dpi_factor`.
    #[inline]
    pub fn to_physical(&self, dpi_factor: f64) -> PhysicalSize {
        (self.width * dpi_factor, self.height * dpi_factor).into()
    }
}
