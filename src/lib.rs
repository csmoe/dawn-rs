mod dawn {
    use crate::sys;
    include!(concat!(env!("OUT_DIR"), "/dawn.rs"));
}

#[allow(nonstandard_style)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/dawn-capi.rs"));
}
