use anyhow::Result;
//use ndarray::{Dimension, IntoDimension};

pub trait AddAssign<R> {
    fn add_assign(&mut self, rhs: R) -> Result<()>;
}

#[cfg(feature = "neural-network")]
pub(crate) struct Im2ColConv2Options {
    pub(crate) filter: [usize; 2],
    pub(crate) padding: [usize; 2],
    pub(crate) stride: [usize; 2],
    pub(crate) dilation: [usize; 2],
}

#[cfg(feature = "neural-network")]
impl Default for Im2ColConv2Options {
    fn default() -> Self {
        Self {
            filter: [0, 0],
            padding: [0, 0],
            stride: [1, 1],
            dilation: [1, 1],
        }
    }
}

#[cfg(feature = "neural-network")]
impl Im2ColConv2Options {
    pub(crate) fn output_shape(&self, input_shape: [usize; 2]) -> [usize; 2] {
        let mut shape = input_shape;
        for ((a, f), (s, (p, d))) in shape.iter_mut().zip(self.filter).zip(
            self.stride
                .into_iter()
                .zip(self.padding.into_iter().zip(self.dilation)),
        ) {
            *a = (*a + 2 * p - d * (f - 1) - 1) / s + 1;
        }
        shape
    }
}

#[cfg(feature = "neural-network")]
pub(crate) trait Im2ColConv2 {
    type Output;
    fn im2col_conv2(&self, options: Im2ColConv2Options) -> Result<Self::Output>;
}

/*
/// Dot (matrix) product.
pub(crate) trait Dot<R> {
    /// Type of the output.
    type Output;
    /// Computes the dot product `self` * `rhs`.
    fn dot(&self, rhs: &R) -> Result<Self::Output>;
}
*/
/*
pub(crate) trait Add<R>: Sized {
    type Output;
    fn add(self, rhs: R) -> Result<Self::Output>;
}

pub(crate) trait Assign<R> {
    fn assign(&mut self, rhs: R) -> Result<()>;
}*/

/*pub(crate) trait AddAssign<R> /*: Add<R> + Assign<R>*/ {
    fn add_assign(&mut self, rhs: &R) -> Result<()>;
}

pub(crate) trait ScaledAdd<T, R>: AddAssign<R> {
    fn scaled_add(&mut self, alpha: T, rhs: &R) -> Result<()>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(unused)]
pub(crate) enum KernelKind {
    Convolution,
    CrossCorrelation,
}

impl KernelKind {
    #[allow(unused)]
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Convolution => "convolution",
            Self::CrossCorrelation => "cross_correlation",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct KernelArgs<D: Dimension> {
    pub(crate) strides: D,
    pub(crate) padding: D,
    pub(crate) dilation: D,
}

impl<D: Dimension> Default for KernelArgs<D> {
    fn default() -> Self {
        let mut strides = D::zeros(D::NDIM.unwrap_or(0));
        strides.slice_mut().iter_mut().for_each(|x| *x = 1);
        let padding = D::zeros(D::NDIM.unwrap_or(0));
        let mut dilation = D::zeros(D::NDIM.unwrap_or(0));
        dilation.slice_mut().iter_mut().for_each(|x| *x = 1);
        Self {
            strides,
            padding,
            dilation,
        }
    }
}

#[cfg(feature = "tensor")]
impl<D: Dimension> KernelArgs<D> {
    pub(crate) fn im2col_shape<E>(&self, input_shape: E, kernel: &D) -> D
    where
        E: IntoDimension<Dim = D>,
    {
        let mut shape = input_shape.into_dimension();
        let shape_iter = shape.slice_mut().iter_mut();
        let kernel = kernel.slice().iter().copied();
        let strides = self.strides.slice().iter().copied();
        let padding = self.padding.slice().iter().copied();
        let dilation = self.dilation.slice().iter().copied();
        for ((a, k), (s, (p, d))) in shape_iter
            .zip(kernel)
            .zip(strides.zip(padding.zip(dilation)))
        {
            *a = (*a + 2 * p - d * (k - 1) - 1) / s + 1;
        }
        shape
    }
    /*pub(crate) fn col2im_shape<E>(&self, input_shape: E, kernel: &D) -> D
        where E: IntoDimension<Dim = D> {
        let mut shape = input_shape.into_dimension();
        let shape_iter = shape.slice_mut().iter_mut();
        let kernel = kernel.slice().iter().copied();
        let strides = self.strides.slice().iter().copied();
        let padding = self.padding.slice().iter().copied();
        let dilation = self.dilation.slice().iter().copied();
        for ((a, k), (s, (p, d))) in shape_iter.zip(kernel).zip(strides.zip(padding.zip(dilation))) {
            let size = *a * 2 * p;
            *a = if size >= p {
                (size - p) / s + 1;
            } else {
                1
            };
        }
        shape
    }*/
}

#[cfg(feature = "ndarray")]
pub(crate) trait Im2Col<D: Dimension> {
    type Output;
    fn im2col(&self, kernel: &D, kind: KernelKind, args: &KernelArgs<D>) -> Result<Self::Output>;
}

#[cfg(feature = "ndarray")]
pub(crate) trait Col2Im<D: Dimension> {
    type Output;
    fn col2im(
        &self,
        shape: &D,
        kernel: &D,
        kind: KernelKind,
        args: &KernelArgs<D>,
    ) -> Result<Self::Output>;
}
*/
