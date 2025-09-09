use std::marker::PhantomData;

use windows::Win32::Foundation::{E_INVALIDARG, E_NOTIMPL};
use windows_core::{implement, Error, IInspectable, Ref, Result, HSTRING};

use crate::Activatable;
use crate::Microsoft::UI::Xaml::Markup::{IXamlMember, IXamlType, IXamlType_Impl};
use crate::Windows::UI::Xaml as WUX;

#[implement(IXamlType)]
pub struct XamlSystemBaseType {
    full_name: HSTRING,
    type_kind: WUX::Interop::TypeKind,
}

impl XamlSystemBaseType {
    pub fn new<T: AsRef<str>>(full_name: T, type_kind: WUX::Interop::TypeKind) -> Self {
        Self {
            full_name: HSTRING::from(full_name.as_ref()),
            type_kind,
        }
    }
}

impl IXamlType_Impl for XamlSystemBaseType_Impl {
    fn BaseType(&self) -> Result<IXamlType> {
        Err(E_NOTIMPL.into())
    }

    fn ContentProperty(&self) -> Result<IXamlMember> {
        Err(E_NOTIMPL.into())
    }

    fn FullName(&self) -> Result<HSTRING> {
        Ok(self.full_name.clone())
    }

    fn IsArray(&self) -> Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn IsCollection(&self) -> Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn IsConstructible(&self) -> Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn IsDictionary(&self) -> Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn IsMarkupExtension(&self) -> Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn IsBindable(&self) -> Result<bool> {
        Err(E_NOTIMPL.into())
    }

    fn ItemType(&self) -> Result<IXamlType> {
        Err(E_NOTIMPL.into())
    }

    fn KeyType(&self) -> Result<IXamlType> {
        Err(E_NOTIMPL.into())
    }

    fn BoxedType(&self) -> Result<IXamlType> {
        Err(E_NOTIMPL.into())
    }

    fn UnderlyingType(&self) -> Result<WUX::Interop::TypeName> {
        Ok(WUX::Interop::TypeName {
            Name: self.full_name.clone(),
            Kind: self.type_kind,
        })
    }

    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }

    fn CreateFromString(&self, _value: &HSTRING) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }

    fn GetMember(&self, _name: &HSTRING) -> Result<IXamlMember> {
        Err(E_NOTIMPL.into())
    }

    fn AddToVector(&self, _instance: Ref<IInspectable>, _value: Ref<IInspectable>) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn AddToMap(
        &self,
        _instance: Ref<IInspectable>,
        _key: Ref<IInspectable>,
        _value: Ref<IInspectable>,
    ) -> Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn RunInitializer(&self) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

#[implement(IXamlType)]
pub struct XamlCustomType<'a, T>
where
    T: Activatable,
{
    full_name: HSTRING,
    base_type: IXamlType,
    _phantom_data: PhantomData<&'a T>,
}

impl<T: Activatable> XamlCustomType<'_, T> {
    pub fn for_page(full_name: &HSTRING) -> Result<IXamlType> {
        let xaml_base_type = XamlSystemBaseType::new(
            "Microsoft.UI.Xaml.Controls.Page",
            WUX::Interop::TypeKind::Metadata,
        );
        let xaml_page_type = Self {
            full_name: full_name.clone(),
            base_type: xaml_base_type.into(),
            _phantom_data: PhantomData,
        };
        Ok(xaml_page_type.into())
    }
}

impl<T: Activatable> IXamlType_Impl for XamlCustomType_Impl<'_, T> {
    fn BaseType(&self) -> Result<IXamlType> {
        Ok(self.base_type.clone())
    }

    fn ContentProperty(&self) -> Result<IXamlMember> {
        Err(Error::empty())
    }

    fn FullName(&self) -> Result<HSTRING> {
        Ok(self.full_name.clone())
    }

    fn IsArray(&self) -> Result<bool> {
        Ok(false)
    }

    fn IsCollection(&self) -> Result<bool> {
        Ok(false)
    }

    fn IsConstructible(&self) -> Result<bool> {
        Ok(true)
    }

    fn IsDictionary(&self) -> Result<bool> {
        Ok(false)
    }

    fn IsMarkupExtension(&self) -> Result<bool> {
        Ok(false)
    }

    fn IsBindable(&self) -> Result<bool> {
        Ok(false)
    }

    fn ItemType(&self) -> Result<IXamlType> {
        Err(Error::empty())
    }

    fn KeyType(&self) -> Result<IXamlType> {
        Err(Error::empty())
    }

    fn BoxedType(&self) -> Result<IXamlType> {
        Err(Error::empty())
    }

    fn UnderlyingType(&self) -> Result<WUX::Interop::TypeName> {
        Ok(WUX::Interop::TypeName {
            Name: self.full_name.clone(),
            Kind: WUX::Interop::TypeKind::Custom,
        })
    }

    fn ActivateInstance(&self) -> Result<IInspectable> {
        T::activate()
    }

    fn CreateFromString(&self, value: &HSTRING) -> Result<IInspectable> {
        Err(Error::new(E_INVALIDARG, value.to_string_lossy()))
    }

    fn GetMember(&self, _name: &HSTRING) -> Result<IXamlMember> {
        Err(Error::empty())
    }

    fn AddToVector(&self, _instance: Ref<IInspectable>, _value: Ref<IInspectable>) -> Result<()> {
        Ok(())
    }

    fn AddToMap(
        &self,
        _instance: Ref<IInspectable>,
        _key: Ref<IInspectable>,
        _value: Ref<IInspectable>,
    ) -> Result<()> {
        Ok(())
    }

    fn RunInitializer(&self) -> Result<()> {
        // TODO: maybe we need to initialize some statics here?
        Ok(())
    }
}
