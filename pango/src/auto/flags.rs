// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use bitflags::bitflags;
use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};
use std::fmt;

bitflags! {
    #[doc(alias = "PangoFontMask")]
    pub struct FontMask: u32 {
        #[doc(alias = "PANGO_FONT_MASK_FAMILY")]
        const FAMILY = ffi::PANGO_FONT_MASK_FAMILY as _;
        #[doc(alias = "PANGO_FONT_MASK_STYLE")]
        const STYLE = ffi::PANGO_FONT_MASK_STYLE as _;
        #[doc(alias = "PANGO_FONT_MASK_VARIANT")]
        const VARIANT = ffi::PANGO_FONT_MASK_VARIANT as _;
        #[doc(alias = "PANGO_FONT_MASK_WEIGHT")]
        const WEIGHT = ffi::PANGO_FONT_MASK_WEIGHT as _;
        #[doc(alias = "PANGO_FONT_MASK_STRETCH")]
        const STRETCH = ffi::PANGO_FONT_MASK_STRETCH as _;
        #[doc(alias = "PANGO_FONT_MASK_SIZE")]
        const SIZE = ffi::PANGO_FONT_MASK_SIZE as _;
        #[doc(alias = "PANGO_FONT_MASK_GRAVITY")]
        const GRAVITY = ffi::PANGO_FONT_MASK_GRAVITY as _;
        #[doc(alias = "PANGO_FONT_MASK_VARIATIONS")]
        const VARIATIONS = ffi::PANGO_FONT_MASK_VARIATIONS as _;
    }
}

impl fmt::Display for FontMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FontMask {
    type GlibType = ffi::PangoFontMask;

    #[inline]
    fn into_glib(self) -> ffi::PangoFontMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoFontMask> for FontMask {
    #[inline]
    unsafe fn from_glib(value: ffi::PangoFontMask) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for FontMask {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_font_mask_get_type()) }
    }
}

impl glib::HasParamSpec for FontMask {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

impl glib::value::ValueType for FontMask {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for FontMask {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for FontMask {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<FontMask> for glib::Value {
    #[inline]
    fn from(v: FontMask) -> Self {
        ToValue::to_value(&v)
    }
}

#[cfg(any(feature = "v1_50"))]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "PangoLayoutDeserializeFlags")]
    pub struct LayoutDeserializeFlags: u32 {
        #[doc(alias = "PANGO_LAYOUT_DESERIALIZE_DEFAULT")]
        const DEFAULT = ffi::PANGO_LAYOUT_DESERIALIZE_DEFAULT as _;
        #[doc(alias = "PANGO_LAYOUT_DESERIALIZE_CONTEXT")]
        const CONTEXT = ffi::PANGO_LAYOUT_DESERIALIZE_CONTEXT as _;
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl fmt::Display for LayoutDeserializeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
#[doc(hidden)]
impl IntoGlib for LayoutDeserializeFlags {
    type GlibType = ffi::PangoLayoutDeserializeFlags;

    #[inline]
    fn into_glib(self) -> ffi::PangoLayoutDeserializeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoLayoutDeserializeFlags> for LayoutDeserializeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::PangoLayoutDeserializeFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl StaticType for LayoutDeserializeFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_layout_deserialize_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl glib::HasParamSpec for LayoutDeserializeFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl glib::value::ValueType for LayoutDeserializeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
unsafe impl<'a> FromValue<'a> for LayoutDeserializeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl ToValue for LayoutDeserializeFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl From<LayoutDeserializeFlags> for glib::Value {
    #[inline]
    fn from(v: LayoutDeserializeFlags) -> Self {
        ToValue::to_value(&v)
    }
}

#[cfg(any(feature = "v1_50"))]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
    #[doc(alias = "PangoLayoutSerializeFlags")]
    pub struct LayoutSerializeFlags: u32 {
        #[doc(alias = "PANGO_LAYOUT_SERIALIZE_DEFAULT")]
        const DEFAULT = ffi::PANGO_LAYOUT_SERIALIZE_DEFAULT as _;
        #[doc(alias = "PANGO_LAYOUT_SERIALIZE_CONTEXT")]
        const CONTEXT = ffi::PANGO_LAYOUT_SERIALIZE_CONTEXT as _;
        #[doc(alias = "PANGO_LAYOUT_SERIALIZE_OUTPUT")]
        const OUTPUT = ffi::PANGO_LAYOUT_SERIALIZE_OUTPUT as _;
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl fmt::Display for LayoutSerializeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
#[doc(hidden)]
impl IntoGlib for LayoutSerializeFlags {
    type GlibType = ffi::PangoLayoutSerializeFlags;

    #[inline]
    fn into_glib(self) -> ffi::PangoLayoutSerializeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoLayoutSerializeFlags> for LayoutSerializeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::PangoLayoutSerializeFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl StaticType for LayoutSerializeFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_layout_serialize_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl glib::HasParamSpec for LayoutSerializeFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl glib::value::ValueType for LayoutSerializeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
unsafe impl<'a> FromValue<'a> for LayoutSerializeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl ToValue for LayoutSerializeFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_50"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl From<LayoutSerializeFlags> for glib::Value {
    #[inline]
    fn from(v: LayoutSerializeFlags) -> Self {
        ToValue::to_value(&v)
    }
}

#[cfg(any(feature = "v1_44"))]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "PangoShapeFlags")]
    pub struct ShapeFlags: u32 {
        #[doc(alias = "PANGO_SHAPE_NONE")]
        const NONE = ffi::PANGO_SHAPE_NONE as _;
        #[doc(alias = "PANGO_SHAPE_ROUND_POSITIONS")]
        const ROUND_POSITIONS = ffi::PANGO_SHAPE_ROUND_POSITIONS as _;
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl fmt::Display for ShapeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl IntoGlib for ShapeFlags {
    type GlibType = ffi::PangoShapeFlags;

    #[inline]
    fn into_glib(self) -> ffi::PangoShapeFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoShapeFlags> for ShapeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::PangoShapeFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl StaticType for ShapeFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_shape_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl glib::HasParamSpec for ShapeFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl glib::value::ValueType for ShapeFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
unsafe impl<'a> FromValue<'a> for ShapeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl ToValue for ShapeFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl From<ShapeFlags> for glib::Value {
    #[inline]
    fn from(v: ShapeFlags) -> Self {
        ToValue::to_value(&v)
    }
}

#[cfg(any(feature = "v1_44"))]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "PangoShowFlags")]
    pub struct ShowFlags: u32 {
        #[doc(alias = "PANGO_SHOW_NONE")]
        const NONE = ffi::PANGO_SHOW_NONE as _;
        #[doc(alias = "PANGO_SHOW_SPACES")]
        const SPACES = ffi::PANGO_SHOW_SPACES as _;
        #[doc(alias = "PANGO_SHOW_LINE_BREAKS")]
        const LINE_BREAKS = ffi::PANGO_SHOW_LINE_BREAKS as _;
        #[doc(alias = "PANGO_SHOW_IGNORABLES")]
        const IGNORABLES = ffi::PANGO_SHOW_IGNORABLES as _;
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl fmt::Display for ShowFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl IntoGlib for ShowFlags {
    type GlibType = ffi::PangoShowFlags;

    #[inline]
    fn into_glib(self) -> ffi::PangoShowFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
#[doc(hidden)]
impl FromGlib<ffi::PangoShowFlags> for ShowFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::PangoShowFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl StaticType for ShowFlags {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::pango_show_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl glib::HasParamSpec for ShowFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name| Self::ParamSpec::builder(name)
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl glib::value::ValueType for ShowFlags {
    type Type = Self;
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
unsafe impl<'a> FromValue<'a> for ShowFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl ToValue for ShowFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_44"))]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
impl From<ShowFlags> for glib::Value {
    #[inline]
    fn from(v: ShowFlags) -> Self {
        ToValue::to_value(&v)
    }
}
