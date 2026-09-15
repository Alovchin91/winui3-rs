use core::cell::UnsafeCell;

use windows::Win32::Foundation::{E_NOINTERFACE, E_POINTER};
use windows_core::{
    Array, AsImpl, ComObject, ComObjectInner, ComObjectInterface, IInspectable, IInspectable_Vtbl,
    IUnknown, IUnknownImpl, Interface, InterfaceRef, Ref, Result, GUID, HRESULT, HSTRING,
};

use crate::Microsoft::UI::Xaml::{
    Application, IApplicationOverrides, IApplicationOverrides_Impl, LaunchActivatedEventArgs,
    Markup::{IXamlMetadataProvider, IXamlMetadataProvider_Impl, IXamlType, XmlnsDefinition},
    XamlTypeInfo::XamlControlsXamlMetaDataProvider,
};
use crate::Windows::UI::Xaml as WUX;

#[allow(non_snake_case)]
pub trait XamlAppOverrides {
    fn OnLaunched(&self, base: &Application, args: Option<&LaunchActivatedEventArgs>)
        -> Result<()>;

    #[cfg(feature = "XamlApp_Navigation")]
    fn TryResolveXamlType(&self, full_name: &HSTRING) -> Result<IXamlType>;
}

pub struct XamlApp<T>
where
    T: XamlAppOverrides,
{
    inner: T,
    provider: XamlControlsXamlMetaDataProvider,
}

impl<T: XamlAppOverrides> XamlApp<T> {
    /// Composes a native Application that may retain the callback state.
    pub fn compose(inner: T) -> Result<Application>
    where
        T: 'static,
    {
        let app = Self {
            inner,
            provider: XamlControlsXamlMetaDataProvider::new()?,
        };
        Application::IApplicationFactory(|this| {
            // SAFETY: compose_with supplies live, initially empty owned out slots
            // and keeps the controlling outer alive throughout CreateInstance.
            app.compose_with(|outer, base, result| unsafe {
                (Interface::vtable(this).CreateInstance)(
                    Interface::as_raw(this),
                    Interface::as_raw(outer),
                    base.cast(),
                    core::ptr::from_mut(result).cast(),
                )
                .ok()
            })
        })
    }

    fn compose_with(
        self,
        factory: impl FnOnce(
            &IInspectable,
            *mut Option<IInspectable>,
            &mut Option<Application>,
        ) -> Result<()>,
    ) -> Result<Application>
    where
        T: 'static,
    {
        let object = ComObject::new(self);
        // The factory writes the base once, on this thread, through this raw
        // pointer while the outer is shared with native code; reentrant QI may
        // observe it before or after that write. No exclusive borrow spans the
        // call, and the base is immutable once the factory returns.
        let base = object.base.get();
        // Consuming the ComObject keeps the allocation alive through `outer`.
        let outer: IInspectable = object.into_interface();
        // Declared after outer so an owned result is released first on failure.
        // The stored base is non-delegating; only the returned class delegates
        // its lifetime to the controlling outer, avoiding a reference cycle.
        let mut result = None;
        factory(&outer, base, &mut result)?;
        result.ok_or_else(windows_core::Error::empty)
    }
}

impl<T: XamlAppOverrides> IApplicationOverrides_Impl for XamlApp_Impl<T> {
    fn OnLaunched(&self, args: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        let inspectable: InterfaceRef<'_, IInspectable> = self.as_interface_ref();
        let base = inspectable.cast()?;
        self.inner.OnLaunched(&base, args.as_ref())
    }
}

impl<T: XamlAppOverrides> IXamlMetadataProvider_Impl for XamlApp_Impl<T> {
    #[cfg(feature = "XamlApp_Navigation")]
    fn GetXamlType(&self, type_name: &WUX::Interop::TypeName) -> Result<IXamlType> {
        let page_resolve = match type_name.Kind {
            WUX::Interop::TypeKind::Custom => self.inner.TryResolveXamlType(&type_name.Name),
            _ => Err(windows_core::Error::empty()),
        };
        page_resolve.or_else(|_| self.provider.GetXamlType(type_name))
    }

    #[cfg(not(feature = "XamlApp_Navigation"))]
    fn GetXamlType(&self, type_name: &WUX::Interop::TypeName) -> Result<IXamlType> {
        self.provider.GetXamlType(type_name)
    }

    #[cfg(feature = "XamlApp_Navigation")]
    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        self.inner
            .TryResolveXamlType(full_name)
            .or_else(|_| self.provider.GetXamlTypeByFullName(full_name))
    }

    #[cfg(not(feature = "XamlApp_Navigation"))]
    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        self.provider.GetXamlTypeByFullName(full_name)
    }

    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        self.provider.GetXmlnsDefinitions()
    }
}

// --- Generated implementation ---

impl<T: XamlAppOverrides> XamlApp<T> {
    #[inline(always)]
    fn into_outer(self) -> XamlApp_Impl<T> {
        XamlApp_Impl::<T> {
            identity: &XamlApp_Impl::<T>::VTABLE_IDENTITY,
            iapplicationoverrides: &XamlApp_Impl::<T>::VTABLE_IAPPLICATIONOVERRIDES,
            ixamlmetadataprovider: &XamlApp_Impl::<T>::VTABLE_IXAMLMETADATAPROVIDER,
            base: UnsafeCell::new(None),
            this: self,
            count: ::windows_core::imp::WeakRefCount::new(),
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct XamlApp_Impl<T: XamlAppOverrides> {
    identity: &'static IInspectable_Vtbl,
    iapplicationoverrides: &'static <IApplicationOverrides as Interface>::Vtable,
    ixamlmetadataprovider: &'static <IXamlMetadataProvider as Interface>::Vtable,
    // Declared before `this` so the final release drops the non-delegating
    // inner before the Rust callback state, matching the release-74 layout.
    base: UnsafeCell<Option<IInspectable>>,
    this: XamlApp<T>,
    count: ::windows_core::imp::WeakRefCount,
}

impl<T: XamlAppOverrides> core::ops::Deref for XamlApp_Impl<T> {
    type Target = XamlApp<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.this
    }
}

impl<T: XamlAppOverrides> XamlApp_Impl<T> {
    const VTABLE_IDENTITY: IInspectable_Vtbl =
        IInspectable_Vtbl::new::<XamlApp_Impl<T>, IApplicationOverrides, 0>();
    const VTABLE_IAPPLICATIONOVERRIDES: <IApplicationOverrides as Interface>::Vtable =
        <IApplicationOverrides as Interface>::Vtable::new::<XamlApp_Impl<T>, -1isize>();
    const VTABLE_IXAMLMETADATAPROVIDER: <IXamlMetadataProvider as Interface>::Vtable =
        <IXamlMetadataProvider as Interface>::Vtable::new::<XamlApp_Impl<T>, -2isize>();
}

impl<T: XamlAppOverrides> IUnknownImpl for XamlApp_Impl<T> {
    type Impl = XamlApp<T>;

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
                    || iid == <::windows_core::imp::IAgileObject as Interface>::IID
                {
                    break 'found &self.identity as *const _ as *const core::ffi::c_void;
                }
                if <IApplicationOverrides as Interface>::Vtable::matches(&iid) {
                    break 'found &self.iapplicationoverrides as *const _
                        as *const core::ffi::c_void;
                }
                if <IXamlMetadataProvider as Interface>::Vtable::matches(&iid) {
                    break 'found &self.ixamlmetadataprovider as *const _
                        as *const core::ffi::c_void;
                }
                if iid == <::windows_core::imp::IMarshal as Interface>::IID {
                    return ::windows_core::imp::marshaler(self.to_interface(), interface);
                }
                let tear_off_ptr = self.count.query(&iid, &self.identity as *const _ as *mut _);
                if !tear_off_ptr.is_null() {
                    *interface = tear_off_ptr;
                    return HRESULT(0);
                }
                // SAFETY: composition initializes this slot once on the calling
                // thread; it remains immutable afterward. UnsafeCell permits
                // the native write while this outer is queried reentrantly.
                if let Some(base) = &*self.base.get() {
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

impl<T: XamlAppOverrides> ComObjectInner for XamlApp<T> {
    type Outer = XamlApp_Impl<T>;

    fn into_object(self) -> ComObject<Self> {
        let boxed = Box::<XamlApp_Impl<T>>::new(self.into_outer());
        unsafe {
            let ptr = Box::into_raw(boxed);
            ComObject::from_raw(core::ptr::NonNull::new_unchecked(ptr))
        }
    }
}

impl<T: XamlAppOverrides> From<XamlApp<T>> for IUnknown {
    #[inline(always)]
    fn from(this: XamlApp<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlAppOverrides> From<XamlApp<T>> for IInspectable {
    #[inline(always)]
    fn from(this: XamlApp<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlAppOverrides> From<XamlApp<T>> for IApplicationOverrides {
    #[inline(always)]
    fn from(this: XamlApp<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlAppOverrides> From<XamlApp<T>> for IXamlMetadataProvider {
    #[inline(always)]
    fn from(this: XamlApp<T>) -> Self {
        let com_object = ComObject::new(this);
        com_object.into_interface()
    }
}

impl<T: XamlAppOverrides> ComObjectInterface<IUnknown> for XamlApp_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IUnknown> {
        unsafe {
            let interface_ptr = &self.identity;
            core::mem::transmute(interface_ptr)
        }
    }
}

impl<T: XamlAppOverrides> ComObjectInterface<IInspectable> for XamlApp_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IInspectable> {
        unsafe {
            let interface_ptr = &self.identity;
            core::mem::transmute(interface_ptr)
        }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<T: XamlAppOverrides> ComObjectInterface<IApplicationOverrides> for XamlApp_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IApplicationOverrides> {
        unsafe { core::mem::transmute(&self.iapplicationoverrides) }
    }
}

#[allow(clippy::needless_lifetimes)]
impl<T: XamlAppOverrides> ComObjectInterface<IXamlMetadataProvider> for XamlApp_Impl<T> {
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IXamlMetadataProvider> {
        unsafe { core::mem::transmute(&self.ixamlmetadataprovider) }
    }
}

impl<T: XamlAppOverrides> AsImpl<XamlApp<T>> for IApplicationOverrides {
    #[inline(always)]
    unsafe fn as_impl_ptr(&self) -> core::ptr::NonNull<XamlApp<T>> {
        unsafe {
            let this = Interface::as_raw(self);
            let this = (this as *mut *mut core::ffi::c_void).sub(1) as *mut XamlApp_Impl<T>;
            core::ptr::NonNull::new_unchecked(&raw const (*this).this as *mut XamlApp<T>)
        }
    }
}

impl<T: XamlAppOverrides> AsImpl<XamlApp<T>> for IXamlMetadataProvider {
    #[inline(always)]
    unsafe fn as_impl_ptr(&self) -> core::ptr::NonNull<XamlApp<T>> {
        unsafe {
            let this = Interface::as_raw(self);
            let this =
                (this as *mut *mut core::ffi::c_void).sub(1 + 1usize) as *mut XamlApp_Impl<T>;
            core::ptr::NonNull::new_unchecked(&raw const (*this).this as *mut XamlApp<T>)
        }
    }
}
