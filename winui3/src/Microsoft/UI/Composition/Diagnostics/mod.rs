// Bindings generated by `windows-bindgen` 0.59.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionDebugHeatMaps(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionDebugHeatMaps,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CompositionDebugHeatMaps {
    pub fn Hide<P0>(&self, subtree: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Hide)(
                windows_core::Interface::as_raw(this),
                subtree.param().abi(),
            )
            .ok()
        }
    }
    pub fn ShowMemoryUsage<P0>(&self, subtree: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ShowMemoryUsage)(
                windows_core::Interface::as_raw(this),
                subtree.param().abi(),
            )
            .ok()
        }
    }
    pub fn ShowOverdraw<P0>(
        &self,
        subtree: P0,
        contentkinds: CompositionDebugOverdrawContentKinds,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ShowOverdraw)(
                windows_core::Interface::as_raw(this),
                subtree.param().abi(),
                contentkinds,
            )
            .ok()
        }
    }
    pub fn ShowRedraw<P0>(&self, subtree: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Visual>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ShowRedraw)(
                windows_core::Interface::as_raw(this),
                subtree.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for CompositionDebugHeatMaps {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionDebugHeatMaps>();
}
unsafe impl windows_core::Interface for CompositionDebugHeatMaps {
    type Vtable = <ICompositionDebugHeatMaps as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionDebugHeatMaps as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionDebugHeatMaps {
    const NAME: &'static str = "Microsoft.UI.Composition.Diagnostics.CompositionDebugHeatMaps";
}
unsafe impl Send for CompositionDebugHeatMaps {}
unsafe impl Sync for CompositionDebugHeatMaps {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: Self = Self(0u32);
    pub const OffscreenRendered: Self = Self(1u32);
    pub const Colors: Self = Self(2u32);
    pub const Effects: Self = Self(4u32);
    pub const Shadows: Self = Self(8u32);
    pub const Lights: Self = Self(16u32);
    pub const Surfaces: Self = Self(32u32);
    pub const SwapChains: Self = Self(64u32);
    pub const All: Self = Self(4294967295u32);
}
impl windows_core::TypeKind for CompositionDebugOverdrawContentKinds {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CompositionDebugOverdrawContentKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Composition.Diagnostics.CompositionDebugOverdrawContentKinds;u4)",
    );
}
impl CompositionDebugOverdrawContentKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CompositionDebugOverdrawContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CompositionDebugOverdrawContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionDebugSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    CompositionDebugSettings,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl CompositionDebugSettings {
    pub fn HeatMaps(&self) -> windows_core::Result<CompositionDebugHeatMaps> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HeatMaps)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryGetSettings<P0>(compositor: P0) -> windows_core::Result<CompositionDebugSettings>
    where
        P0: windows_core::Param<super::Compositor>,
    {
        Self::ICompositionDebugSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetSettings)(
                windows_core::Interface::as_raw(this),
                compositor.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICompositionDebugSettingsStatics<
        R,
        F: FnOnce(&ICompositionDebugSettingsStatics) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            CompositionDebugSettings,
            ICompositionDebugSettingsStatics,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompositionDebugSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ICompositionDebugSettings>();
}
unsafe impl windows_core::Interface for CompositionDebugSettings {
    type Vtable = <ICompositionDebugSettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionDebugSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionDebugSettings {
    const NAME: &'static str = "Microsoft.UI.Composition.Diagnostics.CompositionDebugSettings";
}
unsafe impl Send for CompositionDebugSettings {}
unsafe impl Sync for CompositionDebugSettings {}
windows_core::imp::define_interface!(
    ICompositionDebugHeatMaps,
    ICompositionDebugHeatMaps_Vtbl,
    0x815016b8_f645_5c55_87b5_fe2167282b6f
);
impl windows_core::RuntimeType for ICompositionDebugHeatMaps {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for ICompositionDebugHeatMaps {
    const NAME: &'static str = "Microsoft.UI.Composition.Diagnostics.ICompositionDebugHeatMaps";
}
pub trait ICompositionDebugHeatMaps_Impl: windows_core::IUnknownImpl {
    fn Hide(&self, subtree: windows_core::Ref<'_, super::Visual>) -> windows_core::Result<()>;
    fn ShowMemoryUsage(
        &self,
        subtree: windows_core::Ref<'_, super::Visual>,
    ) -> windows_core::Result<()>;
    fn ShowOverdraw(
        &self,
        subtree: windows_core::Ref<'_, super::Visual>,
        contentKinds: CompositionDebugOverdrawContentKinds,
    ) -> windows_core::Result<()>;
    fn ShowRedraw(&self, subtree: windows_core::Ref<'_, super::Visual>)
        -> windows_core::Result<()>;
}
impl ICompositionDebugHeatMaps_Vtbl {
    pub const fn new<Identity: ICompositionDebugHeatMaps_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Hide<
            Identity: ICompositionDebugHeatMaps_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            subtree: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDebugHeatMaps_Impl::Hide(this, core::mem::transmute_copy(&subtree))
                    .into()
            }
        }
        unsafe extern "system" fn ShowMemoryUsage<
            Identity: ICompositionDebugHeatMaps_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            subtree: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDebugHeatMaps_Impl::ShowMemoryUsage(
                    this,
                    core::mem::transmute_copy(&subtree),
                )
                .into()
            }
        }
        unsafe extern "system" fn ShowOverdraw<
            Identity: ICompositionDebugHeatMaps_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            subtree: *mut core::ffi::c_void,
            contentkinds: CompositionDebugOverdrawContentKinds,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDebugHeatMaps_Impl::ShowOverdraw(
                    this,
                    core::mem::transmute_copy(&subtree),
                    contentkinds,
                )
                .into()
            }
        }
        unsafe extern "system" fn ShowRedraw<
            Identity: ICompositionDebugHeatMaps_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            subtree: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionDebugHeatMaps_Impl::ShowRedraw(
                    this,
                    core::mem::transmute_copy(&subtree),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICompositionDebugHeatMaps,
                OFFSET,
            >(),
            Hide: Hide::<Identity, OFFSET>,
            ShowMemoryUsage: ShowMemoryUsage::<Identity, OFFSET>,
            ShowOverdraw: ShowOverdraw::<Identity, OFFSET>,
            ShowRedraw: ShowRedraw::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDebugHeatMaps as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ICompositionDebugHeatMaps_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Hide: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShowMemoryUsage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ShowOverdraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        CompositionDebugOverdrawContentKinds,
    ) -> windows_core::HRESULT,
    pub ShowRedraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionDebugSettings,
    ICompositionDebugSettings_Vtbl,
    0xf4c0c0f6_7f5f_5014_a0d6_c8c7eeecace6
);
impl windows_core::RuntimeType for ICompositionDebugSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for ICompositionDebugSettings {
    const NAME: &'static str = "Microsoft.UI.Composition.Diagnostics.ICompositionDebugSettings";
}
pub trait ICompositionDebugSettings_Impl: windows_core::IUnknownImpl {
    fn HeatMaps(&self) -> windows_core::Result<CompositionDebugHeatMaps>;
}
impl ICompositionDebugSettings_Vtbl {
    pub const fn new<Identity: ICompositionDebugSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HeatMaps<
            Identity: ICompositionDebugSettings_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositionDebugSettings_Impl::HeatMaps(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICompositionDebugSettings,
                OFFSET,
            >(),
            HeatMaps: HeatMaps::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDebugSettings as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ICompositionDebugSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HeatMaps: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ICompositionDebugSettingsStatics,
    ICompositionDebugSettingsStatics_Vtbl,
    0xb56f8aab_2b8c_51aa_b974_10e5c517f50e
);
impl windows_core::RuntimeType for ICompositionDebugSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl windows_core::RuntimeName for ICompositionDebugSettingsStatics {
    const NAME: &'static str =
        "Microsoft.UI.Composition.Diagnostics.ICompositionDebugSettingsStatics";
}
pub trait ICompositionDebugSettingsStatics_Impl: windows_core::IUnknownImpl {
    fn TryGetSettings(
        &self,
        compositor: windows_core::Ref<'_, super::Compositor>,
    ) -> windows_core::Result<CompositionDebugSettings>;
}
impl ICompositionDebugSettingsStatics_Vtbl {
    pub const fn new<Identity: ICompositionDebugSettingsStatics_Impl, const OFFSET: isize>() -> Self
    {
        unsafe extern "system" fn TryGetSettings<
            Identity: ICompositionDebugSettingsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            compositor: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositionDebugSettingsStatics_Impl::TryGetSettings(
                    this,
                    core::mem::transmute_copy(&compositor),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                ICompositionDebugSettingsStatics,
                OFFSET,
            >(),
            TryGetSettings: TryGetSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionDebugSettingsStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct ICompositionDebugSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetSettings: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
