/*
mod dawn {
    use crate::sys;
    include!(concat!(env!("OUT_DIR"), "/dawn.rs"));
}

#[cfg(feature = "dawn_wire")]
mod dawn_wire {
    use crate::sys;
    include!(concat!(env!("OUT_DIR"), "/dawn-wire.rs"));
}
*/

#[allow(nonstandard_style)]
#[allow(unused)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/dawn-capi.rs"));
}

#[cfg(feature = "dawn_wire")]
#[allow(nonstandard_style)]
#[allow(unused)]
mod wire_sys {
    include!(concat!(env!("OUT_DIR"), "/dawn-wire-capi.rs"));
}
