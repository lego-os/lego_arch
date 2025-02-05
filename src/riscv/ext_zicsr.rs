/// Zicsr 拓展

/// 读取csr
#[macro_export]
macro_rules! csrr {
    ($csr:ident) => {{
        use core::arch::asm;
        let mut ret:usize;
        unsafe{
            asm!("csrr {0},{1}", out(reg) ret,const $csr,options(nomem, nostack));
        }
        ret
    }};
    ($csr:ident,$xlen:ty) => {{
        use core::arch::asm;
        let mut ret:$xlen;
        unsafe{
            asm!("csrr {0},{1}", out(reg) ret,const $csr,options(nomem, nostack));
        }
        ret
    }};
}

/// 写入csr
#[macro_export]
macro_rules! csrw {
    ($csr:ident,$val:ident) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrw {0}, {1}",const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 写入csr，立即数版本
#[macro_export]
macro_rules! csrwi {
    ($csr:ident,$val:expr) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrwi {0}, {1}",const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 置位csr
#[macro_export]
macro_rules! csrs {
    ($csr:ident,$val:ident) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrs {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 置位csr，立即数版本
#[macro_export]
macro_rules! csrsi {
    ($csr:ident,$val:expr) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrsi {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 清除位csr
#[macro_export]
macro_rules! csrc {
    ($csr:ident,$val:ident) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrc {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}

/// 清除位csr，立即数版本
#[macro_export]
macro_rules! csrci {
    ($csr:ident,$val:expr) => {{
        use core::arch::asm;
        unsafe {
            asm!("csrci {0}, {1}", const $csr,in(reg) $val,options(nomem, nostack));
        }
    }};
}
