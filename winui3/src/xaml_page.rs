use windows::Win32::Foundation::{E_NOINTERFACE, E_POINTER};
use windows_core::{
    AsImpl, ComObject, ComObjectInner, ComObjectInterface, IInspectable, IInspectable_Vtbl,
    IUnknown, IUnknownImpl, Interface, InterfaceRef, Ref, Result, GUID, HRESULT,
};

use crate::Microsoft::UI::Xaml::{
    Controls::{IPageOverrides, IPageOverrides_Impl, Page},
    Navigation::{NavigatingCancelEventArgs, NavigationEventArgs},
};

#[allow(non_snake_case)]
pub trait XamlPageOverrides {
    fn OnNavigatedFrom(&self, base: &Page, args: Option<&NavigationEventArgs>) -> Result<()>;

    fn OnNavigatedTo(&self, base: &Page, args: Option<&NavigationEventArgs>) -> Result<()>;

    fn OnNavigatingFrom(&self, base: &Page, args: Option<&NavigatingCancelEventArgs>)
        -> Result<()>;
}

pub struct XamlPage<T>
where
    T: XamlPageOverrides,
{
    inner: T,
}

impl<T: XamlPageOverrides> XamlPage<T> {
    pub fn compose(inner: T) -> Result<Page> {
        let xaml_page = Self { inner };
        Page::IPageFactory(|this| unsafe {
            let outer: IInspectable = xaml_page.into();
            let outer__ = Interface::as_raw(&outer);
            // IInspectable Vtable is the identity, and it's the first field
            // of the _Impl struct, so it can be directly cast it to _Impl.
            // See QueryInterface.
            let r#impl = outer__ as *mut XamlPage_Impl<T>;
            let base__ = &mut (*r#impl).base;
            let mut result__ = core::mem::zeroed();
            (Interface::vtable(this).CreateInstance)(
                Interface::as_raw(this),
                outer__,
                base__ as *mut _ as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
}

impl<T: XamlPageOverrides> IPageOverrides_Impl for XamlPage_Impl<T> {
    fn OnNavigatedFrom(&self, args: Ref<NavigationEventArgs>) -> Result<()> {
        let inspectable: InterfaceRef<'_, IInspectable> = self.as_interface_ref();
        let base = inspectable.cast()?;
        self.inner.OnNavigatedFrom(&base, args.as_ref())
    }

    fn OnNavigatedTo(&self, args: Ref<NavigationEventArgs>) -> Result<()> {
        let inspectable: InterfaceRef<'_, IInspectable> = self.as_interface_ref();
        let base = inspectable.cast()?;
        self.inner.OnNavigatedTo(&base, args.as_ref())
    }

    fn OnNavigatingFrom(&self, args: Ref<NavigatingCancelEventArgs>) -> Result<()> {
        let inspectable: InterfaceRef<'_, IInspectable> = self.as_interface_ref();
        let base = inspectable.cast()?;
        self.inner.OnNavigatingFrom(&base, args.as_ref())
    }
}

// --- Generated implementation ---

impl<T: XamlPageOverrides> XamlPage<T> {
    #[inline(always)]
    fn into_outer(self) -> XamlPage_Impl<T> {
        XamlPage_Impl::<T> {
            identity: &XamlPage_Impl::<T>::VTABLE_IDENTITY,
            ipageoverrides: &XamlPage_Impl::<T>::VTABLE_IPAGEOVERRIDES,
            count: windows_core::imp::WeakRefCount::new(),
            this: self,
            base: Option::None,
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct XamlPage_Impl<T: XamlPageOverrides> {
    identity: &'static IInspectable_Vtbl,
    ipageoverrides: &'static <IPageOverrides as Interface>::Vtable,
    this: XamlPage<T>,
    base: Option<IInspectable>,
    count: windows_core::imp::WeakRefCount,
}

impl<T: XamlPageOverrides> core::ops::Deref for XamlPage_Impl<T> {
    type Target = XamlPage<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

impl<T: XamlPageOverrides> XamlPage_Impl<T> {
    const VTABLE_IDENTITY: IInspectable_Vtbl =
        IInspectable_Vtbl::new::<XamlPage_Impl<T>, IPageOverrides, 0>();
    const VTABLE_IPAGEOVERRIDES: <IPageOverrides as Interface>::Vtable =
        <IPageOverrides as Interface>::Vtable::new::<XamlPage_Impl<T>, -1isize>();
}

impl<T: XamlPageOverrides> IUnknownImpl for XamlPage_Impl<T> {
    type Impl = XamlPage<T>;

    #[inline(always)]
    fn get_impl(&self) -> &Self::Impl {
        &self.this
    }

    #[inline(always)]
    fn get_impl_mut(&mut self) -> &mut Self::Impl {
        &mut self.this
    }

    #[inline(always)]
    fn into_inner(self) -> Self::Impl {
        self.this
    }

    #[inline(always)]
    fn AddRef(&self) -> u32 {
        self.count.add_ref()
    }

    #[inline(always)]
    unsafe fn Release(self_: *mut Self) -> u32 {
        let remaining = (*self_).count.release();
        if remaining == 0 {
            _ = Box::from_raw(self_);
        }
        remaining
    }

    #[inline(always)]
    fn is_reference_count_one(&self) -> bool {
        self.count.is_one()
    }

    unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT {
        if value.is_null() {
            return E_POINTER;
        }
        *value = 0;
        HRESULT(0)
    }

    fn to_object(&self) -> ComObject<Self::Impl> {
        self.count.add_ref();
        unsafe {
            ComObject::from_raw(core::ptr::NonNull::new_unchecked(
                self as *const Self as *mut Self,
            ))
        }
    }

    unsafe fn QueryInterface(
        &self,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT {
        unsafe {
            if iid.is_null() || interface.is_null() {
                return E_POINTER;
            }
            let iid = *iid;
            let interface_ptr: *const core::ffi::c_void = 'found: {
                if iid == <IUnknown as Interface>::IID
                    || iid == <IInspectable as Interface>::IID
                    || iid == <windows_core::imp::IAgileObject as Interface>::IID
                {
                    break 'found &self.identity as *const _ as *const core::ffi::c_void;
                }
                if <IPageOverrides as Interface>::Vtable::matches(&iid) {
                    break 'found &self.ipageoverrides as *const _ as *const core::ffi::c_void;
                }
                #[cfg(windows)]
                if iid == <windows_core::imp::IMarshal as Interface>::IID {
                    return windows_core::imp::marshaler(self.to_interface(), interface);
                }
                let tear_off_ptr = self.count.query(&iid, &self.identity as *const _ as *mut _);
                if !tear_off_ptr.is_null() {
                    *interface = tear_off_ptr;
                    return HRESULT(0);
                }
                if let Some(base) = &self.base {
                    return Interface::query(base, &iid, interface);
                }
                *interface = core::ptr::null_mut();
                return E_NOINTERFACE;
            };
            debug_assert!(!interface_ptr.is_null());
            *interface = interface_ptr as *mut core::ffi::c_void;
            self.count.add_ref();
            HRESULT(0)
        }
    }
}

impl<T: XamlPageOverrides> ComObjectInner for XamlPage<T> {
    type Outer = XamlPage_Impl<T>;

    fn into_object(self) -> ComObject<Self> {
        let boxed = Box::<XamlPage_Impl<T>>::new(self.into_outer());
        unsafe {
            let ptr = Box::into_raw(boxed);
            ComObject::from_raw(core::ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl<T: XamlPageOverrides> From<XamlPage<T>> for IUnknown {
    #[inline(always)]
    fn from(this: XamlPage<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlPageOverrides> From<XamlPage<T>> for IInspectable {
    #[inline(always)]
    fn from(this: XamlPage<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlPageOverrides> From<XamlPage<T>> for IPageOverrides {
    #[inline(always)]
    fn from(this: XamlPage<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlPageOverrides> ComObjectInterface<IUnknown> for XamlPage_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IUnknown> {
        unsafe {
            let interface_ptr = &self.identity;
            core::mem::transmute(interface_ptr)
        }
    }
}

impl<T: XamlPageOverrides> ComObjectInterface<IInspectable> for XamlPage_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IInspectable> {
        unsafe {
            let interface_ptr = &self.identity;
            core::mem::transmute(interface_ptr)
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<T: XamlPageOverrides> ComObjectInterface<IPageOverrides> for XamlPage_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IPageOverrides> {
        unsafe { core::mem::transmute(&self.ipageoverrides) }
    }
}

impl<T: XamlPageOverrides> AsImpl<XamlPage<T>> for IPageOverrides {
    // SAFETY: the offset is guaranteed to be in bounds, and the implementation struct
    // is guaranteed to live at least as long as `self`.
    #[inline(always)]
    unsafe fn as_impl_ptr(&self) -> core::ptr::NonNull<XamlPage<T>> {
        unsafe {
            let this = Interface::as_raw(self);
            // Subtract away the vtable offset plus 1, for the `identity` field, to get
            // to the impl struct which contains that original implementation type.
            let this = (this as *mut *mut core::ffi::c_void).sub(1) as *mut XamlPage_Impl<T>;
            core::ptr::NonNull::new_unchecked(&raw const (*this).this as *mut XamlPage<T>)
        }
    }
}
