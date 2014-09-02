// Copyright (c) 2014 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NOTE Remove this once the next rust nightly is out.
pub mod libc;

#[cfg(target_os = "freebsd")]
#[cfg(target_os = "macos")]
#[cfg(windows)]
pub mod bpf;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(windows)]
pub mod winpcap;
