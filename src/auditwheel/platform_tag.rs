use crate::auditwheel::Policy;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Decides how to handle manylinux and musllinux compliance
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Copy)]
pub enum PlatformTag {
    /// Use the manylinux_x_y tag
    Manylinux {
        /// GLIBC version major
        x: u16,
        /// GLIBC version minor
        y: u16,
    },
    /// Use the musllinux_x_y tag
    Musllinux {
        /// musl libc version major
        x: u16,
        /// musl libc version minor
        y: u16,
    },
    /// Use the native linux tag
    Linux,
}

impl PlatformTag {
    /// `manylinux1` aka `manylinux_2_5`
    pub fn manylinux1() -> Self {
        Self::Manylinux { x: 2, y: 5 }
    }

    /// `manylinux2010` aka `manylinux_2_12`
    pub fn manylinux2010() -> Self {
        Self::Manylinux { x: 2, y: 12 }
    }

    /// `manylinux2014` aka `manylinux_2_17`
    pub fn manylinux2014() -> Self {
        Self::Manylinux { x: 2, y: 17 }
    }

    /// manylinux aliases
    pub fn aliases(&self) -> Vec<String> {
        match self {
            PlatformTag::Manylinux { .. } => {
                if let Some(policy) = Policy::from_name(&self.to_string()) {
                    policy.aliases
                } else {
                    Vec::new()
                }
            }
            PlatformTag::Musllinux { .. } => Vec::new(),
            PlatformTag::Linux => Vec::new(),
        }
    }
}

impl fmt::Display for PlatformTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlatformTag::Manylinux { x, y } => write!(f, "manylinux_{}_{}", x, y),
            PlatformTag::Musllinux { x, y } => write!(f, "musllinux_{}_{}", x, y),
            PlatformTag::Linux => write!(f, "linux"),
        }
    }
}

impl FromStr for PlatformTag {
    type Err = &'static str;

    fn from_str(value: &str) -> anyhow::Result<Self, Self::Err> {
        let value = value.to_ascii_lowercase();
        match value.as_str() {
            "off" | "linux" => Ok(PlatformTag::Linux),
            "auto" | "1" | "manylinux1" => Ok(PlatformTag::manylinux1()),
            "2010" | "manylinux2010" => Ok(PlatformTag::manylinux2010()),
            "2014" | "manylinux2014" => Ok(PlatformTag::manylinux2014()),
            _ => {
                if let Some(value) = value.strip_prefix("musllinux_") {
                    let mut parts = value.split('_');
                    let x = parts
                        .next()
                        .and_then(|x| x.parse::<u16>().ok())
                        .ok_or("invalid musllinux option")?;
                    let y = parts
                        .next()
                        .and_then(|y| y.parse::<u16>().ok())
                        .ok_or("invalid musllinux option")?;
                    Ok(PlatformTag::Musllinux { x, y })
                } else {
                    let value = value.strip_prefix("manylinux_").unwrap_or(&value);
                    let mut parts = value.split('_');
                    let x = parts
                        .next()
                        .and_then(|x| x.parse::<u16>().ok())
                        .ok_or("invalid manylinux option")?;
                    let y = parts
                        .next()
                        .and_then(|y| y.parse::<u16>().ok())
                        .ok_or("invalid manylinux option")?;
                    Ok(PlatformTag::Manylinux { x, y })
                }
            }
        }
    }
}
