//! # nano-fs-perms
//!
//! File access permissions modelled after [POSIX permission bits].
//!
//! # Examples
//!
//! ```
//! use nano_fs_perms::Perms;
//!
//! let perms = Perms::OWNER_READ | Perms::OWNER_WRITE | Perms::GROUP_READ | Perms::OTHERS_READ;
//! assert_eq!(perms.to_string(), "rw-r--r--");
//!
//! let perms: u32 = perms.into();
//! assert_eq!(perms, 0o644);
//! ```
//!
//! [POSIX permission bits]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/sys_stat.h.html

#![no_std]

#[cfg(feature = "std")]
extern crate std;

use core::{convert::TryFrom, fmt, ops};

/// File access permissions.
///
/// Instances can be built using bitwise operations on the various consts
/// provided by the structure.
///
/// Alternatively, use the `TryFrom` trait to attempt to create an instance
/// from a primitive integer.
///
/// `Perms` can be deconstructed back to primitive integers using the `From`
/// trait.
///
/// # Examples
///
/// Constructing file access permissions using consts:
///
/// ```
/// use nano_fs_perms::Perms;
///
/// let perms = Perms::OWNER_READ | Perms::OWNER_WRITE | Perms::GROUP_READ | Perms::OTHERS_READ;
/// assert_eq!(perms.to_string(), "rw-r--r--");
///
/// let perms: u32 = perms.into();
/// assert_eq!(perms, 0o644);
/// ```
///
/// Constructing file access permissions using integers:
///
/// ```
/// use std::convert::TryFrom;
/// use nano_fs_perms::Perms;
///
/// let perms = Perms::try_from(0o755).unwrap();
/// assert_eq!(perms.to_string(), "rwxr-xr-x");
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Perms(u32);

impl From<Perms> for u32 {
    fn from(perms: Perms) -> Self {
        perms.0
    }
}

/// The error type returned when a checked file access permissions type
/// conversion fails.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PermsTryFromError(());

impl fmt::Display for PermsTryFromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid file access permissions type conversion attempted")
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for PermsTryFromError {}

impl TryFrom<u32> for Perms {
    type Error = PermsTryFromError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value & !Perms::MASK.0 != 0 {
            Err(PermsTryFromError(()))
        } else {
            Ok(Self(value))
        }
    }
}

impl Perms {
    /// No permission bits.
    ///
    /// **Value**: `0`
    pub const NONE: Self = Self(0);

    /// File owner has read permission.
    ///
    /// **Value**: `0o400`
    ///
    /// **POSIX**: `S_IRUSR`
    pub const OWNER_READ: Self = Self(0o400);

    /// File owner has write permission.
    ///
    /// **Value**: `0o200`
    ///
    /// **POSIX**: `S_IWUSR`
    pub const OWNER_WRITE: Self = Self(0o200);

    /// File owner has execute/search permission.
    ///
    /// **Value**: `0o100`
    ///
    /// **POSIX**: `S_IXUSR`
    pub const OWNER_EXEC: Self = Self(0o100);

    /// File owner has read, write, and execute/search permissions.
    ///
    /// Equivalent to `OWNER_READ | OWNER_WRITE | OWNER_EXEC`.
    ///
    /// **Value**: `0o700`
    ///
    /// **POSIX**: `S_IRWXU`
    pub const OWNER_ALL: Self = Self(0o70);

    /// The file's user group has read permission.
    ///
    /// **Value**: `0o40`
    ///
    /// **POSIX**: `S_IRGRP`
    pub const GROUP_READ: Self = Self(0o40);

    /// The file's user group has write permission.
    ///
    /// **Value**: `0o20`
    ///
    /// **POSIX**: `S_IWGRP`
    pub const GROUP_WRITE: Self = Self(0o20);

    /// The file's user group has execute/search permission.
    ///
    /// **Value**: `0o10`
    ///
    /// **POSIX**: `S_IXGRP`
    pub const GROUP_EXEC: Self = Self(0o10);

    /// The file's user group has read, write, and execute/search permissions.
    ///
    /// Equivalent to `GROUP_READ | GROUP_WRITE | GROUP_EXEC`.
    ///
    /// **Value**: `0o70`
    ///
    /// **POSIX**: `S_IRWXG`
    pub const GROUP_ALL: Self = Self(0o70);

    /// Other users have read permissions.
    ///
    /// **Value**: `0o4`
    ///
    /// **POSIX**: `S_IROTH`
    pub const OTHERS_READ: Self = Self(0o4);

    /// Other users have write permissions.
    ///
    /// **Value**: `0o2`
    ///
    /// **POSIX**: `S_IWOTH`
    pub const OTHERS_WRITE: Self = Self(0o2);

    /// Other users have execute/search permissions.
    ///
    /// **Value**: `0o1`
    ///
    /// **POSIX**: `S_IXOTH`
    pub const OTHERS_EXEC: Self = Self(0o1);

    /// Other users have read, write, and execute/search permissions.
    ///
    /// Equivalent to `OTHERS_READ | OTHERS_WRITE | OTHERS_EXEC`.
    ///
    /// **Value**: `0o7`
    ///
    /// **POSIX**: `S_IRWXO`
    pub const OTHERS_ALL: Self = Self(0o7);

    /// All users have read, write, and execute/search permissions.
    ///
    /// Equivalent to `OWNER_ALL | GROUP_ALL | OTHERS_ALL`.
    ///
    /// **Value**: `0o777`
    pub const ALL: Self = Self(0o777);

    /// Set user ID to file owner user ID on execution.
    ///
    /// **Value**: `0o4000`
    ///
    /// **POSIX**: `S_ISUID`
    pub const SET_UID: Self = Self(0o4000);

    /// Set group ID to file's user group ID on execution.
    ///
    /// **Value**: `0o2000`
    ///
    /// **POSIX**: `S_ISGID`
    pub const SET_GID: Self = Self(0o2000);

    /// Implementation defined sticky bit.
    ///
    /// POSIX XSI specifies that when set on a directory, only file owners may
    /// delete files even if the directory is writeable to others (used with
    /// `/tmp`).
    ///
    /// **Value**: `0o1000`
    ///
    /// **POSIX**: `S_ISVTX`
    pub const STICKY_BIT: Self = Self(0o1000);

    /// All valid permission bits.
    ///
    /// Equivalent to `ALL | SET_UID | SET_GID | STICKY_BIT`.
    ///
    /// **Value**: `0o7777`
    pub const MASK: Self = Self(0o7777);

    /// Returns `true` if the value is a valid representation of file access
    /// permissions.
    ///
    /// This is shorthand for:
    ///
    /// ```ignore
    /// Perms::try_from(/* value */).is_ok()
    /// ```
    pub fn is_valid(value: u32) -> bool {
        Self::try_from(value).is_ok()
    }
}

impl fmt::Display for Perms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = ["-"; 9];

        if self & Perms::OWNER_READ == Perms::OWNER_READ {
            repr[0] = "r";
        }

        if self & Perms::OWNER_WRITE == Perms::OWNER_WRITE {
            repr[1] = "w";
        }

        match (
            self & Perms::OWNER_EXEC == Perms::OWNER_EXEC,
            self & Perms::SET_UID == Perms::SET_UID,
        ) {
            (true, false) => repr[2] = "x",
            (true, true) => repr[2] = "s",
            (false, true) => repr[2] = "S",
            (false, false) => (),
        }

        if self & Perms::GROUP_READ == Perms::GROUP_READ {
            repr[3] = "r";
        }

        if self & Perms::GROUP_WRITE == Perms::GROUP_WRITE {
            repr[4] = "w";
        }

        match (
            self & Perms::GROUP_EXEC == Perms::GROUP_EXEC,
            self & Perms::SET_GID == Perms::SET_GID,
        ) {
            (true, false) => repr[5] = "x",
            (true, true) => repr[5] = "s",
            (false, true) => repr[5] = "S",
            (false, false) => (),
        }

        if self & Perms::OTHERS_READ == Perms::OTHERS_READ {
            repr[6] = "r";
        }

        if self & Perms::OTHERS_WRITE == Perms::OTHERS_WRITE {
            repr[7] = "w";
        }

        match (
            self & Perms::OTHERS_EXEC == Perms::OTHERS_EXEC,
            self & Perms::STICKY_BIT == Perms::STICKY_BIT,
        ) {
            (true, false) => repr[8] = "x",
            (true, true) => repr[8] = "t",
            (false, true) => repr[8] = "T",
            (false, false) => (),
        }

        for bit in repr.into_iter() {
            f.write_str(bit)?;
        }

        Ok(())
    }
}

impl ops::BitAnd for Perms {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitAnd<&'_ Perms> for Perms {
    type Output = Self;

    fn bitand(self, rhs: &'_ Self) -> Self::Output {
        self & *rhs
    }
}

impl ops::BitAnd for &'_ Perms {
    type Output = Perms;

    fn bitand(self, rhs: Self) -> Self::Output {
        *self & *rhs
    }
}

impl ops::BitAnd<Perms> for &'_ Perms {
    type Output = Perms;

    fn bitand(self, rhs: Perms) -> Self::Output {
        *self & rhs
    }
}

impl ops::BitAndAssign for Perms {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Self(self.0 & rhs.0);
    }
}

impl ops::BitAndAssign<&'_ Perms> for Perms {
    fn bitand_assign(&mut self, rhs: &'_ Self) {
        *self = Self(self.0 & rhs.0)
    }
}

impl ops::BitOr for Perms {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitOr<&'_ Perms> for Perms {
    type Output = Self;

    fn bitor(self, rhs: &'_ Self) -> Self::Output {
        self | *rhs
    }
}

impl ops::BitOr for &'_ Perms {
    type Output = Perms;

    fn bitor(self, rhs: Self) -> Self::Output {
        *self | *rhs
    }
}

impl ops::BitOr<Perms> for &'_ Perms {
    type Output = Perms;

    fn bitor(self, rhs: Perms) -> Self::Output {
        *self | rhs
    }
}

impl ops::BitOrAssign for Perms {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0);
    }
}

impl ops::BitOrAssign<&'_ Perms> for Perms {
    fn bitor_assign(&mut self, rhs: &'_ Self) {
        *self = Self(self.0 | rhs.0)
    }
}

impl ops::BitXor for Perms {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ops::BitXor<&'_ Perms> for Perms {
    type Output = Self;

    fn bitxor(self, rhs: &'_ Self) -> Self::Output {
        self ^ *rhs
    }
}

impl ops::BitXor for &'_ Perms {
    type Output = Perms;

    fn bitxor(self, rhs: Self) -> Self::Output {
        *self ^ *rhs
    }
}

impl ops::BitXor<Perms> for &'_ Perms {
    type Output = Perms;

    fn bitxor(self, rhs: Perms) -> Self::Output {
        *self ^ rhs
    }
}

impl ops::BitXorAssign for Perms {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl ops::BitXorAssign<&'_ Perms> for Perms {
    fn bitxor_assign(&mut self, rhs: &'_ Self) {
        *self = Self(self.0 ^ rhs.0)
    }
}
