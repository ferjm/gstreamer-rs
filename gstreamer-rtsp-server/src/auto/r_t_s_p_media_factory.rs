// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use RTSPAddressPool;
use RTSPMedia;
use RTSPPublishClockMode;
use RTSPSuspendMode;
use RTSPTransportMode;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use gst;
use gst_rtsp;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RTSPMediaFactory(Object<ffi::GstRTSPMediaFactory, ffi::GstRTSPMediaFactoryClass>);

    match fn {
        get_type => || ffi::gst_rtsp_media_factory_get_type(),
    }
}

impl RTSPMediaFactory {
    pub fn new() -> RTSPMediaFactory {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_new())
        }
    }
}

impl Default for RTSPMediaFactory {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactory {}
unsafe impl Sync for RTSPMediaFactory {}

pub trait RTSPMediaFactoryExt {
    //fn add_role(&self, role: &str, fieldname: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn construct(&self, url: &gst_rtsp::RTSPUrl) -> Option<RTSPMedia>;

    fn create_element(&self, url: &gst_rtsp::RTSPUrl) -> Option<gst::Element>;

    fn get_address_pool(&self) -> Option<RTSPAddressPool>;

    fn get_buffer_size(&self) -> u32;

    fn get_clock(&self) -> Option<gst::Clock>;

    fn get_latency(&self) -> u32;

    fn get_launch(&self) -> Option<String>;

    fn get_media_gtype(&self) -> glib::types::Type;

    fn get_multicast_iface(&self) -> Option<String>;

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions>;

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile;

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans;

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode;

    fn get_retransmission_time(&self) -> gst::ClockTime;

    fn get_suspend_mode(&self) -> RTSPSuspendMode;

    fn get_transport_mode(&self) -> RTSPTransportMode;

    fn is_eos_shutdown(&self) -> bool;

    fn is_shared(&self) -> bool;

    fn is_stop_on_disonnect(&self) -> bool;

    fn set_address_pool<'a, P: Into<Option<&'a RTSPAddressPool>>>(&self, pool: P);

    fn set_buffer_size(&self, size: u32);

    fn set_clock<'a, P: IsA<gst::Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q);

    fn set_eos_shutdown(&self, eos_shutdown: bool);

    fn set_latency(&self, latency: u32);

    fn set_launch(&self, launch: &str);

    fn set_media_gtype(&self, media_gtype: glib::types::Type);

    fn set_multicast_iface<'a, P: Into<Option<&'a str>>>(&self, multicast_iface: P);

    //fn set_permissions<'a, P: Into<Option<&'a /*Ignored*/RTSPPermissions>>>(&self, permissions: P);

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile);

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans);

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode);

    fn set_retransmission_time(&self, time: gst::ClockTime);

    fn set_shared(&self, shared: bool);

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool);

    fn set_suspend_mode(&self, mode: RTSPSuspendMode);

    fn set_transport_mode(&self, mode: RTSPTransportMode);

    fn get_property_eos_shutdown(&self) -> bool;

    fn get_property_shared(&self) -> bool;

    fn get_property_stop_on_disconnect(&self) -> bool;

    fn connect_media_configure<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_media_constructed<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_launch_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RTSPMediaFactory> + IsA<glib::object::Object>> RTSPMediaFactoryExt for O {
    //fn add_role(&self, role: &str, fieldname: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gst_rtsp_media_factory_add_role() }
    //}

    fn construct(&self, url: &gst_rtsp::RTSPUrl) -> Option<RTSPMedia> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_construct(self.to_glib_none().0, url.to_glib_none().0))
        }
    }

    fn create_element(&self, url: &gst_rtsp::RTSPUrl) -> Option<gst::Element> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_media_factory_create_element(self.to_glib_none().0, url.to_glib_none().0))
        }
    }

    fn get_address_pool(&self) -> Option<RTSPAddressPool> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_address_pool(self.to_glib_none().0))
        }
    }

    fn get_buffer_size(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_media_factory_get_buffer_size(self.to_glib_none().0)
        }
    }

    fn get_clock(&self) -> Option<gst::Clock> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_clock(self.to_glib_none().0))
        }
    }

    fn get_latency(&self) -> u32 {
        unsafe {
            ffi::gst_rtsp_media_factory_get_latency(self.to_glib_none().0)
        }
    }

    fn get_launch(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_launch(self.to_glib_none().0))
        }
    }

    fn get_media_gtype(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_media_gtype(self.to_glib_none().0))
        }
    }

    fn get_multicast_iface(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_get_multicast_iface(self.to_glib_none().0))
        }
    }

    //fn get_permissions(&self) -> /*Ignored*/Option<RTSPPermissions> {
    //    unsafe { TODO: call ffi::gst_rtsp_media_factory_get_permissions() }
    //}

    fn get_profiles(&self) -> gst_rtsp::RTSPProfile {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_profiles(self.to_glib_none().0))
        }
    }

    fn get_protocols(&self) -> gst_rtsp::RTSPLowerTrans {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_protocols(self.to_glib_none().0))
        }
    }

    fn get_publish_clock_mode(&self) -> RTSPPublishClockMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_publish_clock_mode(self.to_glib_none().0))
        }
    }

    fn get_retransmission_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_retransmission_time(self.to_glib_none().0))
        }
    }

    fn get_suspend_mode(&self) -> RTSPSuspendMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_suspend_mode(self.to_glib_none().0))
        }
    }

    fn get_transport_mode(&self) -> RTSPTransportMode {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_get_transport_mode(self.to_glib_none().0))
        }
    }

    fn is_eos_shutdown(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_eos_shutdown(self.to_glib_none().0))
        }
    }

    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_shared(self.to_glib_none().0))
        }
    }

    fn is_stop_on_disonnect(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_media_factory_is_stop_on_disonnect(self.to_glib_none().0))
        }
    }

    fn set_address_pool<'a, P: Into<Option<&'a RTSPAddressPool>>>(&self, pool: P) {
        let pool = pool.into();
        let pool = pool.to_glib_none();
        unsafe {
            ffi::gst_rtsp_media_factory_set_address_pool(self.to_glib_none().0, pool.0);
        }
    }

    fn set_buffer_size(&self, size: u32) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_buffer_size(self.to_glib_none().0, size);
        }
    }

    fn set_clock<'a, P: IsA<gst::Clock> + 'a, Q: Into<Option<&'a P>>>(&self, clock: Q) {
        let clock = clock.into();
        let clock = clock.to_glib_none();
        unsafe {
            ffi::gst_rtsp_media_factory_set_clock(self.to_glib_none().0, clock.0);
        }
    }

    fn set_eos_shutdown(&self, eos_shutdown: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_eos_shutdown(self.to_glib_none().0, eos_shutdown.to_glib());
        }
    }

    fn set_latency(&self, latency: u32) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_latency(self.to_glib_none().0, latency);
        }
    }

    fn set_launch(&self, launch: &str) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_launch(self.to_glib_none().0, launch.to_glib_none().0);
        }
    }

    fn set_media_gtype(&self, media_gtype: glib::types::Type) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_media_gtype(self.to_glib_none().0, media_gtype.to_glib());
        }
    }

    fn set_multicast_iface<'a, P: Into<Option<&'a str>>>(&self, multicast_iface: P) {
        let multicast_iface = multicast_iface.into();
        let multicast_iface = multicast_iface.to_glib_none();
        unsafe {
            ffi::gst_rtsp_media_factory_set_multicast_iface(self.to_glib_none().0, multicast_iface.0);
        }
    }

    //fn set_permissions<'a, P: Into<Option<&'a /*Ignored*/RTSPPermissions>>>(&self, permissions: P) {
    //    unsafe { TODO: call ffi::gst_rtsp_media_factory_set_permissions() }
    //}

    fn set_profiles(&self, profiles: gst_rtsp::RTSPProfile) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_profiles(self.to_glib_none().0, profiles.to_glib());
        }
    }

    fn set_protocols(&self, protocols: gst_rtsp::RTSPLowerTrans) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_protocols(self.to_glib_none().0, protocols.to_glib());
        }
    }

    fn set_publish_clock_mode(&self, mode: RTSPPublishClockMode) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_publish_clock_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_retransmission_time(&self, time: gst::ClockTime) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_retransmission_time(self.to_glib_none().0, time.to_glib());
        }
    }

    fn set_shared(&self, shared: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_shared(self.to_glib_none().0, shared.to_glib());
        }
    }

    fn set_stop_on_disconnect(&self, stop_on_disconnect: bool) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_stop_on_disconnect(self.to_glib_none().0, stop_on_disconnect.to_glib());
        }
    }

    fn set_suspend_mode(&self, mode: RTSPSuspendMode) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_suspend_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn set_transport_mode(&self, mode: RTSPTransportMode) {
        unsafe {
            ffi::gst_rtsp_media_factory_set_transport_mode(self.to_glib_none().0, mode.to_glib());
        }
    }

    fn get_property_eos_shutdown(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "eos-shutdown".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_shared(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "shared".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_stop_on_disconnect(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stop-on-disconnect".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_media_configure<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &RTSPMedia) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "media-configure",
                transmute(media_configure_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_media_constructed<F: Fn(&Self, &RTSPMedia) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &RTSPMedia) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "media-constructed",
                transmute(media_constructed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_buffer_size_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer-size",
                transmute(notify_buffer_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_clock_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::clock",
                transmute(notify_clock_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_eos_shutdown_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::eos-shutdown",
                transmute(notify_eos_shutdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_latency_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::latency",
                transmute(notify_latency_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_launch_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::launch",
                transmute(notify_launch_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_profiles_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::profiles",
                transmute(notify_profiles_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocols_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocols",
                transmute(notify_protocols_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shared_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shared",
                transmute(notify_shared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stop_on_disconnect_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stop-on-disconnect",
                transmute(notify_stop_on_disconnect_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_suspend_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::suspend-mode",
                transmute(notify_suspend_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_transport_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::transport-mode",
                transmute(notify_transport_mode_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn media_configure_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, object: *mut ffi::GstRTSPMedia, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P, &RTSPMedia) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(object))
}

unsafe extern "C" fn media_constructed_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, object: *mut ffi::GstRTSPMedia, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P, &RTSPMedia) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(object))
}

unsafe extern "C" fn notify_buffer_size_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_clock_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_eos_shutdown_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_latency_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_launch_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_profiles_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocols_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shared_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stop_on_disconnect_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_suspend_mode_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_transport_mode_trampoline<P>(this: *mut ffi::GstRTSPMediaFactory, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<RTSPMediaFactory> {
    callback_guard!();
    let f: &&(Fn(&P) + Send + Sync + 'static) = transmute(f);
    f(&RTSPMediaFactory::from_glib_borrow(this).downcast_unchecked())
}
