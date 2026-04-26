use windows::{
    Foundation::{IPropertyValue_Impl, IReference, IReference_Impl},
    Win32::Foundation::E_NOTIMPL,
};
use windows_core::{implement, RuntimeType};

/// Generic `IReference<T>` implementation for value types that
/// `windows::Foundation::PropertyValue` can't produce.
///
/// For primitive types — `bool`, the integer types, `f32`/`f64`, `HSTRING`,
/// etc. — use `PropertyValue`'s static factory methods (`CreateBoolean`,
/// `CreateUInt32`, `CreateString`, …) instead. `Reference<T>` is the escape
/// hatch for everything `PropertyValue` doesn't support, e.g. `IReference<Color>`
/// for `AppWindowTitleBar::SetButtonBackgroundColor`.
///
/// `IReference<T>` inherits `IPropertyValue`, so this type provides both
/// vtables; the `IPropertyValue` getters all return `E_NOTIMPL` —
/// `Reference<T>` is the complement to `PropertyValue`, not the general case.
#[implement(IReference<T>)]
pub struct Reference<T>
where
    T: RuntimeType + 'static,
{
    value: Option<T>,
}

impl<T: RuntimeType + 'static> Reference<T> {
    pub fn new(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T: RuntimeType + 'static> Default for Reference<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T: RuntimeType + 'static> From<Option<T>> for Reference<T> {
    fn from(value: Option<T>) -> Self {
        Self { value }
    }
}

impl<T: RuntimeType + 'static> windows_core::Param<IReference<T>> for Reference<T> {
    unsafe fn param(self) -> windows_core::ParamValue<IReference<T>> {
        windows_core::ParamValue::Owned(self.into())
    }
}

impl<T: RuntimeType + 'static> IReference_Impl<T> for Reference_Impl<T> {
    fn Value(&self) -> windows_core::Result<T> {
        match &self.value {
            Some(value) => Ok(value.clone()),
            None => Err(windows_core::Error::empty()), // TODO: Verify
        }
    }
}

impl<T: RuntimeType + 'static> IPropertyValue_Impl for Reference_Impl<T> {
    fn Type(&self) -> windows_core::Result<windows::Foundation::PropertyType> {
        Ok(windows::Foundation::PropertyType::OtherType)
    }

    fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt8(&self) -> windows_core::Result<u8> {
        Err(E_NOTIMPL.into())
    }

    fn GetInt16(&self) -> windows_core::Result<i16> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt16(&self) -> windows_core::Result<u16> {
        Err(E_NOTIMPL.into())
    }

    fn GetInt32(&self) -> windows_core::Result<i32> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt32(&self) -> windows_core::Result<u32> {
        Err(E_NOTIMPL.into())
    }

    fn GetInt64(&self) -> windows_core::Result<i64> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt64(&self) -> windows_core::Result<u64> {
        Err(E_NOTIMPL.into())
    }

    fn GetSingle(&self) -> windows_core::Result<f32> {
        Err(E_NOTIMPL.into())
    }

    fn GetDouble(&self) -> windows_core::Result<f64> {
        Err(E_NOTIMPL.into())
    }

    fn GetChar16(&self) -> windows_core::Result<u16> {
        Err(E_NOTIMPL.into())
    }

    fn GetBoolean(&self) -> windows_core::Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        Err(E_NOTIMPL.into())
    }

    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        Err(E_NOTIMPL.into())
    }

    fn GetDateTime(&self) -> windows_core::Result<windows::Foundation::DateTime> {
        Err(E_NOTIMPL.into())
    }

    fn GetTimeSpan(&self) -> windows_core::Result<windows::Foundation::TimeSpan> {
        Err(E_NOTIMPL.into())
    }

    fn GetPoint(&self) -> windows_core::Result<windows::Foundation::Point> {
        Err(E_NOTIMPL.into())
    }

    fn GetSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        Err(E_NOTIMPL.into())
    }

    fn GetRect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt8Array(&self, _value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetInt16Array(&self, _value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt16Array(&self, _value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetInt32Array(&self, _value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt32Array(&self, _value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetInt64Array(&self, _value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetUInt64Array(&self, _value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetSingleArray(&self, _value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetDoubleArray(&self, _value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetChar16Array(&self, _value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetBooleanArray(&self, _value: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetStringArray(
        &self,
        _value: &mut windows_core::Array<windows_core::HSTRING>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetInspectableArray(
        &self,
        _value: &mut windows_core::Array<windows_core::IInspectable>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetGuidArray(
        &self,
        _value: &mut windows_core::Array<windows_core::GUID>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetDateTimeArray(
        &self,
        _value: &mut windows_core::Array<windows::Foundation::DateTime>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetTimeSpanArray(
        &self,
        _value: &mut windows_core::Array<windows::Foundation::TimeSpan>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetPointArray(
        &self,
        _value: &mut windows_core::Array<windows::Foundation::Point>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetSizeArray(
        &self,
        _value: &mut windows_core::Array<windows::Foundation::Size>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn GetRectArray(
        &self,
        _value: &mut windows_core::Array<windows::Foundation::Rect>,
    ) -> windows_core::Result<()> {
        Err(E_NOTIMPL.into())
    }
}
