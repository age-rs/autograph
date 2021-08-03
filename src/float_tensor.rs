use crate::{
    device::Device,
    float::{
        Float, FloatArcBuffer, FloatBuffer, FloatCowBuffer, FloatSlice, FloatSliceMut, FloatType,
    },
    result::Result,
    tensor::{dim_strides_from_shape, into_dimensionality, into_shape, DataBase},
};

use ndarray::{Dimension, IntoDimension, Ix0, Ix1, Ix2, Ix3, Ix4, Ix5, Ix6, IxDyn, ShapeBuilder};
use serde::{Deserialize, Serialize};

//mod linalg;

macro_rules! impl_data_base {
    ($($data:ident $(<$a:lifetime>)?),+) => {
        $(
            impl DataBase for $data $(<$a>)? {
                fn device(&self) -> Device {
                    self.0.device()
                }
                fn len(&self) -> usize {
                    self.0.len()
                }
                fn is_empty(&self) -> bool {
                    self.0.is_empty()
                }
            }
        )+
    };
}

impl_data_base! {FloatOwnedRepr, FloatArcRepr, FloatArcMutRepr<'_>, FloatViewRepr<'_>, FloatViewMutRepr<'_>, FloatCowRepr<'_>}

/// Marker trait for FloatTensorBase representation.
///
/// Typically use [`FloatTensor'] / [`FloatArcTensor`] / [`FloatTensorView`] / [`FloatTensorViewMut`] / [`FloatCowTensor`] types directly.
pub trait FloatData: Sized + DataBase {
    #[doc(hidden)]
    fn try_into_buffer(self) -> Result<FloatBuffer, Self> {
        Err(self)
    }
    #[doc(hidden)]
    fn into_owned(self) -> Result<FloatOwnedRepr> {
        match self.try_into_buffer() {
            Ok(buffer) => Ok(FloatOwnedRepr(buffer)),
            Err(this) => Ok(FloatOwnedRepr(this.as_slice().to_owned()?)),
        }
    }
    #[doc(hidden)]
    fn try_into_arc_buffer(self) -> Result<FloatArcBuffer, Self> {
        self.try_into_buffer().map(Into::into)
    }
    #[doc(hidden)]
    fn into_shared(self) -> Result<FloatArcRepr> {
        match self.try_into_arc_buffer() {
            Ok(buffer) => Ok(FloatArcRepr(buffer)),
            Err(this) => Ok(FloatArcRepr(this.as_slice().to_owned()?.into())),
        }
    }
    #[doc(hidden)]
    fn as_slice(&self) -> FloatSlice;
}

/// Marker trait for owned float tensors [`FloatTensor`] / [`FloatArcTensor`] / [`FloatCowTensor`].
pub trait FloatDataOwned: FloatData {
    #[doc(hidden)]
    fn from_buffer(buffer: FloatBuffer) -> Self;
}

/// Marker trait for mutable float tensors [`FloatTensor`] / [`FloatTensorViewMut`] / [`FloatArcTensorMut`].
pub trait FloatDataMut: FloatData {
    #[doc(hidden)]
    fn as_slice_mut(&mut self) -> FloatSliceMut;
}

/// FloatTensor representation.
#[derive(Debug, Serialize, Deserialize)]
pub struct FloatOwnedRepr(FloatBuffer);

impl FloatData for FloatOwnedRepr {
    fn try_into_buffer(self) -> Result<FloatBuffer, Self> {
        Ok(self.0)
    }
    fn as_slice(&self) -> FloatSlice {
        self.0.as_slice()
    }
}

impl FloatDataOwned for FloatOwnedRepr {
    fn from_buffer(buffer: FloatBuffer) -> Self {
        Self(buffer)
    }
}

impl FloatDataMut for FloatOwnedRepr {
    fn as_slice_mut(&mut self) -> FloatSliceMut {
        self.0.as_slice_mut()
    }
}

/// FloatArcTensor representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatArcRepr(FloatArcBuffer);

impl FloatData for FloatArcRepr {
    fn try_into_buffer(self) -> Result<FloatBuffer, Self> {
        self.0.try_unwrap().map_err(Self)
    }
    fn try_into_arc_buffer(self) -> Result<FloatArcBuffer, Self> {
        Ok(self.0)
    }
    fn as_slice(&self) -> FloatSlice {
        self.0.as_slice()
    }
}

impl FloatDataOwned for FloatArcRepr {
    fn from_buffer(buffer: FloatBuffer) -> Self {
        Self(buffer.into())
    }
}

/// FloatArcTensorMut representation.
#[derive(Debug)]
pub struct FloatArcMutRepr<'a>(&'a mut FloatArcBuffer);

impl FloatData for FloatArcMutRepr<'_> {
    fn as_slice(&self) -> FloatSlice {
        self.0.as_slice()
    }
}

/// FloatTensorView representation.
#[derive(Debug, Clone)]
pub struct FloatViewRepr<'a>(FloatSlice<'a>);

impl FloatData for FloatViewRepr<'_> {
    fn as_slice(&self) -> FloatSlice {
        self.0.as_slice()
    }
}

/// FloatTensorViewMut representation.
#[derive(Debug)]
pub struct FloatViewMutRepr<'a>(FloatSliceMut<'a>);

impl FloatData for FloatViewMutRepr<'_> {
    fn as_slice(&self) -> FloatSlice {
        self.0.as_slice()
    }
}

impl FloatDataMut for FloatViewMutRepr<'_> {
    fn as_slice_mut(&mut self) -> FloatSliceMut {
        self.0.as_slice_mut()
    }
}

/// FloatCowTensor representation.
#[derive(Debug)]
pub struct FloatCowRepr<'a>(FloatCowBuffer<'a>);

impl FloatDataOwned for FloatCowRepr<'_> {
    fn from_buffer(buffer: FloatBuffer) -> Self {
        Self(buffer.into())
    }
}

impl FloatData for FloatCowRepr<'_> {
    fn try_into_buffer(self) -> Result<FloatBuffer, Self> {
        self.0.try_unwrap().map_err(Self)
    }
    fn as_slice(&self) -> FloatSlice {
        self.0.as_slice()
    }
}

/// Dynamically Float typed [`TensorBase`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatTensorBase<S: FloatData, D: Dimension> {
    dim: D,
    strides: D,
    data: S,
}

/// Owned FloatTensor
///
/// See [`FloatTensorBase`].
pub type FloatTensor<D> = FloatTensorBase<FloatOwnedRepr, D>;
/// FloatTensor with 1 element
pub type FloatTensor0 = FloatTensor<Ix0>;
/// FloatTensor with 1 dimension
pub type FloatTensor1 = FloatTensor<Ix1>;
/// FloatTensor with 2 dimensions
pub type FloatTensor2 = FloatTensor<Ix2>;
/// FloatTensor with 3 dimensions
pub type FloatTensor3 = FloatTensor<Ix3>;
/// FloatTensor with 4 dimensions
pub type FloatTensor4 = FloatTensor<Ix4>;
/// FloatTensor with 5 dimensions
pub type FloatTensor5 = FloatTensor<Ix5>;
/// FloatTensor with 6 dimensions
pub type FloatTensor6 = FloatTensor<Ix6>;
/// FloatTensor with dynamic dimensions
pub type FloatTensorD = FloatTensor<IxDyn>;

/// Shared FloatTensor
///
/// See [`FloatTensorBase`].
pub type FloatArcTensor<D> = FloatTensorBase<FloatArcRepr, D>;
/// FloatArcTensor with 1 element
pub type FloatArcTensor0 = FloatArcTensor<Ix0>;
/// FloatArcTensor with 1 dimension
pub type FloatArcTensor1 = FloatArcTensor<Ix1>;
/// FloatArcTensor with 2 dimensions
pub type FloatArcTensor2 = FloatArcTensor<Ix2>;
/// FloatArcTensor with 3 dimensions
pub type FloatArcTensor3 = FloatArcTensor<Ix3>;
/// FloatArcTensor with 4 dimensions
pub type FloatArcTensor4 = FloatArcTensor<Ix4>;
/// FloatArcTensor with 5 dimensions
pub type FloatArcTensor5 = FloatArcTensor<Ix5>;
/// FloatArcTensor with 6 dimensions
pub type FloatArcTensor6 = FloatArcTensor<Ix6>;
/// FloatArcTensor with dynamic dimensions
pub type FloatArcTensorD = FloatArcTensor<IxDyn>;

/// Mutably borrowed ArcFloatTensor
///
/// See [`FloatTensorBase`].
pub type FloatArcTensorMut<D> = FloatTensorBase<FloatArcRepr, D>;
/// FloatArcTensorMut with 1 element
pub type FloatArcTensorMut0 = FloatArcTensorMut<Ix0>;
/// FloatArcTensorMut with 1 dimension
pub type FloatArcTensorMut1 = FloatArcTensorMut<Ix1>;
/// FloatArcTensorMut with 2 dimensions
pub type FloatArcTensorMut2 = FloatArcTensorMut<Ix2>;
/// FloatArcTensorMut with 3 dimensions
pub type FloatArcTensorMut3 = FloatArcTensorMut<Ix3>;
/// FloatArcTensorMut with 4 dimensions
pub type FloatArcTensorMut4 = FloatArcTensorMut<Ix4>;
/// FloatArcTensorMut with 5 dimensions
pub type FloatArcTensorMut5 = FloatArcTensorMut<Ix5>;
/// FloatArcTensorMut with 6 dimensions
pub type FloatArcTensorMut6 = FloatArcTensorMut<Ix6>;
/// FloatArcTensorMut with dynamic dimensions
pub type FloatArcTensorMutD = FloatArcTensorMut<IxDyn>;

/// Borrowed FloatTensor
///
/// See [`FloatTensorBase`].
pub type FloatTensorView<'a, D> = FloatTensorBase<FloatViewRepr<'a>, D>;
/// FloatTensorView with 1 element
pub type FloatTensorView0<'a> = FloatTensorView<'a, Ix0>;
/// FloatTensorView with 1 dimension
pub type FloatTensorView1<'a> = FloatTensorView<'a, Ix1>;
/// FloatTensorView with 2 dimensions
pub type FloatTensorView2<'a> = FloatTensorView<'a, Ix2>;
/// FloatTensorView with 3 dimensions
pub type FloatTensorView3<'a> = FloatTensorView<'a, Ix3>;
/// FloatTensorView with 4 dimensions
pub type FloatTensorView4<'a> = FloatTensorView<'a, Ix4>;
/// FloatTensorView with 5 dimensions
pub type FloatTensorView5<'a> = FloatTensorView<'a, Ix5>;
/// FloatTensorView with 6 dimensions
pub type FloatTensorView6<'a> = FloatTensorView<'a, Ix6>;
/// FloatTensorView with dynamic dimensions
pub type FloatTensorViewD<'a> = FloatTensorView<'a, IxDyn>;

/// Mutably borrowed FloatTensor
///
/// See [`FloatTensorBase`].
pub type FloatTensorViewMut<'a, D> = FloatTensorBase<FloatViewMutRepr<'a>, D>;
/// FloatTensorViewMut with 1 element
pub type FloatTensorViewMut0<'a> = FloatTensorViewMut<'a, Ix0>;
/// FloatTensorViewMut with 1 dimension
pub type FloatTensorViewMut1<'a> = FloatTensorViewMut<'a, Ix1>;
/// FloatTensorViewMut with 2 dimensions
pub type FloatTensorViewMut2<'a> = FloatTensorViewMut<'a, Ix2>;
/// FloatTensorViewMut with 3 dimensions
pub type FloatTensorViewMut3<'a> = FloatTensorViewMut<'a, Ix3>;
/// FloatTensorViewMut with 4 dimensions
pub type FloatTensorViewMut4<'a> = FloatTensorViewMut<'a, Ix4>;
/// FloatTensorViewMut with 5 dimensions
pub type FloatTensorViewMut5<'a> = FloatTensorViewMut<'a, Ix5>;
/// FloatTensorViewMut with 6 dimensions
pub type FloatTensorViewMut6<'a> = FloatTensorViewMut<'a, Ix6>;
/// FloatTensorViewMut with dynamic dimensions
pub type FloatTensorViewMutD<'a> = FloatTensorViewMut<'a, IxDyn>;

/// FloatTensor that is either borrowed or owned.
///
/// See [`FloatTensorBase`].
pub type FloatCowTensor<'a, D> = FloatTensorBase<FloatCowRepr<'a>, D>;
/// FloatCowTensor with 1 element
pub type FloatCowTensor0<'a> = FloatCowTensor<'a, Ix0>;
/// FloatCowTensor with 1 dimension
pub type FloatCowTensor1<'a> = FloatCowTensor<'a, Ix1>;
/// FloatCowTensor with 2 dimensions
pub type FloatCowTensor2<'a> = FloatCowTensor<'a, Ix2>;
/// FloatCowTensor with 3 dimensions
pub type FloatCowTensor3<'a> = FloatCowTensor<'a, Ix3>;
/// FloatCowTensor with 4 dimensions
pub type FloatCowTensor4<'a> = FloatCowTensor<'a, Ix4>;
/// FloatCowTensor with 5 dimensions
pub type FloatCowTensor5<'a> = FloatCowTensor<'a, Ix5>;
/// FloatCowTensor with 6 dimensions
pub type FloatCowTensor6<'a> = FloatCowTensor<'a, Ix6>;
/// FloatCowTensor with dynamic dimensions
pub type FloatCowTensorD<'a> = FloatCowTensor<'a, IxDyn>;

impl<S: FloatData, D: Dimension> FloatTensorBase<S, D> {
    /// Allocates a float tensor of type `float_type` on `device` with `shape`.
    ///
    /// # Safety
    ///
    /// The tensor is not initialized.
    ///
    /// **Errors**
    /// See [`Buffer::alloc()`](crate::device::buffer::BufferBase::alloc()).
    pub unsafe fn alloc<Sh>(float_type: FloatType, device: Device, shape: Sh) -> Result<Self>
    where
        S: FloatDataOwned,
        Sh: ShapeBuilder<Dim = D>,
    {
        let (dim, strides) = dim_strides_from_shape(shape.into_shape());
        let data = S::from_buffer(FloatBuffer::alloc(float_type, device, dim.size())?);
        Ok(Self { dim, strides, data })
    }
    /// Creates a float tensor on `device` with `shape` filled with `elem`.
    ///
    /// **Errors**
    /// See [`Buffer::alloc()`](crate::device::buffer::BufferBase::alloc()).
    pub fn from_elem<T, Sh>(device: Device, shape: Sh, elem: T) -> Result<Self>
    where
        T: Float,
        S: FloatDataOwned,
        Sh: ShapeBuilder<Dim = D>,
    {
        let (dim, strides) = dim_strides_from_shape(shape.into_shape());
        let data = S::from_buffer(FloatBuffer::from_elem(device, dim.size(), elem)?);
        Ok(Self { dim, strides, data })
    }
    /// Creates a float tensor of type `float_type` on `device` with `shape` filled with 0's.
    ///
    /// **Errors**
    /// See [`Buffer::alloc()`](crate::device::buffer::BufferBase::alloc()).
    pub fn zeros<Sh>(float_type: FloatType, device: Device, shape: Sh) -> Result<Self>
    where
        S: FloatDataOwned,
        Sh: ShapeBuilder<Dim = D>,
    {
        let (dim, strides) = dim_strides_from_shape(shape.into_shape());
        let data = S::from_buffer(FloatBuffer::zeros(float_type, device, dim.size())?);
        Ok(Self { dim, strides, data })
    }
    /// The device of the tensor.
    pub fn device(&self) -> Device {
        self.data.device()
    }
    /// The dimensions of the tensor in pattern form.
    pub fn dim(&self) -> D::Pattern {
        self.dim.clone().into_pattern()
    }
    /// The dimensions of the tensor.
    pub fn raw_dim(&self) -> D {
        self.dim.clone()
    }
    /// The dimensions of the tensor as a slice.
    pub fn shape(&self) -> &[usize] {
        self.dim.slice()
    }
    /// The strides of the tensor as a slice.
    pub fn strides(&self) -> &[isize] {
        bytemuck::cast_slice(self.strides.slice())
    }
    /// The length of the tensor.
    pub fn len(&self) -> usize {
        debug_assert_eq!(self.data.len(), self.dim.size());
        self.data.len()
    }
    /// Whether the tensor is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// The dimensionality of the tensor.
    pub fn ndim(&self) -> usize {
        self.dim.ndim()
    }
    /// Converts the tensor into dimension `D2`.
    ///
    /// Typically this is used to downcast from [`IxDyn`](type@ndarray::IxDyn) to a static dimensionality. For conversions to [`IxDyn`](type@ndarray::IxDyn), use [`.into_dyn()`](TensorBase::into_dyn()).
    ///
    /// **Errors**
    /// The number of axes of `D2` must be the same as `D`.
    pub fn into_dimensionality<D2>(self) -> Result<FloatTensorBase<S, D2>>
    where
        D2: Dimension,
    {
        let (dim, strides) = into_dimensionality(&self.dim, &self.strides)?;
        Ok(FloatTensorBase {
            dim,
            strides,
            data: self.data,
        })
    }
    /// Returns the tensor with dim `shape`.
    ///
    /// **Errors**
    /// The tensor must be contiguous, with default strides.
    pub fn into_shape<E>(self, shape: E) -> Result<FloatTensorBase<S, E::Dim>>
    where
        E: IntoDimension,
    {
        let (dim, strides) = into_shape(&self.dim, &self.strides, shape)?;
        Ok(FloatTensorBase {
            dim,
            strides,
            data: self.data,
        })
    }
    /// Converts the dimensionality of the tensor to [`IxDyn`](type@ndarray::IxDyn).
    pub fn into_dyn(self) -> FloatTensorBase<S, IxDyn> {
        FloatTensorBase {
            dim: self.dim.into_dyn(),
            strides: self.strides.into_dyn(),
            data: self.data,
        }
    }
    /// Borrows the tensor as a [`FloatTensorView`].
    pub fn view(&self) -> FloatTensorView<D> {
        FloatTensorView {
            dim: self.dim.clone(),
            strides: self.strides.clone(),
            data: FloatViewRepr(self.data.as_slice()),
        }
    }
    /// Borrows the tensor as a [`FloatTensorViewMut`].
    pub fn view_mut(&mut self) -> FloatTensorViewMut<D>
    where
        S: FloatDataMut,
    {
        FloatTensorViewMut {
            dim: self.dim.clone(),
            strides: self.strides.clone(),
            data: FloatViewMutRepr(self.data.as_slice_mut()),
        }
    }
    /// Reverses (transposes) the axes of the array.
    pub fn reversed_axes(mut self) -> Self {
        self.dim.slice_mut().reverse();
        self.strides.slice_mut().reverse();
        self
    }
    /// Retunrs a view with reversed (transposed) axes.
    pub fn t(&self) -> FloatTensorView<D> {
        self.view().reversed_axes()
    }
    /// Returns a [`FloatCowBuffer`] in standard layout.
    ///
    /// If the data is default strided, ie standard layout (C or RowMajor), borrows the data as a slice. Otherwise, clones the data.
    ///
    /// See also [`as_raw_slice()`](FloatTensorBase::as_raw_slice()).
    ///
    /// **Errors**
    /// See [`.to_owned()`](FloatTensorBase::to_owned()).
    pub fn to_slice(&self) -> Result<FloatCowBuffer> {
        if self.strides == self.dim.default_strides() {
            Ok(self.data.as_slice().into())
        } else {
            Ok(self.data.as_slice().to_owned()?.into())
        }
    }
    /// Borrows the tensor as a [`FloatSlice`].
    ///
    /// # Note
    /// If the tensor is not standard layout (C or RowMajor), this may not be what you want. See [`to_slice()`](TensorBase::to_slice()).
    pub fn as_raw_slice(&self) -> FloatSlice {
        self.data.as_slice()
    }
    /// Mutably borrows the tensor as a [`SliceMut`].
    ///
    /// # Note
    /// If the tensor is not standard layout (C or RowMajor), this may not be what you want.
    pub fn as_raw_slice_mut(&mut self) -> FloatSliceMut
    where
        S: FloatDataMut,
    {
        self.data.as_slice_mut()
    }
    /// Transfers the tensor into the `device`.
    ///
    /// **Errors**
    /// See [`Buffer::into_device()`](crate::device::buffer::BufferBase::into_device()).
    pub async fn into_device(self, device: Device) -> Result<FloatTensor<D>> {
        if device == self.device() {
            self.into_owned()
        } else {
            let buffer = self.data.as_slice().into_device(device).await?;
            Ok(FloatTensor {
                dim: self.dim,
                strides: self.strides,
                data: FloatOwnedRepr(buffer),
            })
        }
    }
    /// Converts into a [`FloatTensor`].
    pub fn into_owned(self) -> Result<FloatTensor<D>> {
        Ok(FloatTensor {
            dim: self.dim,
            strides: self.strides,
            data: self.data.into_owned()?,
        })
    }
    /// Converts into an [`ArcTensor`].
    pub fn into_shared(self) -> Result<FloatArcTensor<D>> {
        Ok(FloatArcTensor {
            dim: self.dim,
            strides: self.strides,
            data: self.data.into_shared()?,
        })
    }
}

impl<S: FloatDataOwned> From<FloatBuffer> for FloatTensorBase<S, Ix1> {
    fn from(buffer: FloatBuffer) -> Self {
        let dim = buffer.len().into_dimension();
        let strides = dim.default_strides();
        let data = S::from_buffer(buffer);
        Self { dim, strides, data }
    }
}