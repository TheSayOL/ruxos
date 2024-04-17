mod auxv;
mod load_elf;
mod stack;

use alloc::vec;
use core::ffi::c_char;

use crate::{
    config,
    imp::stat::{sys_getgid, sys_getuid},
    sys_getegid, sys_geteuid, sys_random,
    utils::char_ptr_to_str,
};

/// int execve(const char *pathname, char *const argv[], char *const envp[] );
pub fn sys_execve(pathname: *const c_char, argv: usize, envp: usize) -> ! {
    use auxv::*;

    let path = char_ptr_to_str(pathname).unwrap();
    let prog = load_elf::ElfProg::new(path);

    // get entry
    let mut entry = prog.entry;

    // if interp is needed
    let mut at_base = 0;
    if !prog.interp_path.is_empty() {
        let interp_path = char_ptr_to_str(prog.interp_path.as_ptr() as _).unwrap();
        let interp_prog = load_elf::ElfProg::new(interp_path);
        entry = interp_prog.entry;
        at_base = interp_prog.base;
        debug!("sys_execve: INTERP base is {:x}", at_base);
    };

    // create stack
    let mut stack = stack::Stack::new();

    // non 8B info
    stack.push(&[0u8; 32], 16);
    let rand = rand();
    let p_rand = stack.push(&rand, 16);

    // auxv
    // TODO: vdso
    let auxv = vec![
        AT_PHDR,
        prog.phdr,
        AT_PHNUM,
        prog.phnum,
        AT_PHENT,
        prog.phent,
        AT_BASE,
        at_base,
        AT_PAGESZ,
        config::PAGE_SIZE_4K,
        AT_HWCAP,
        0,
        AT_PLATFORM,
        platform(),
        AT_CLKTCK,
        100,
        AT_FLAGS,
        0,
        AT_ENTRY,
        prog.entry,
        AT_UID,
        sys_getuid() as usize,
        AT_EUID,
        sys_geteuid() as usize,
        AT_EGID,
        sys_getegid() as usize,
        AT_GID,
        sys_getgid() as usize,
        AT_SECURE,
        0,
        AT_EXECFN,
        pathname as usize,
        AT_RANDOM,
        p_rand,
        AT_SYSINFO_EHDR,
        0,
        AT_IGNORE,
        0,
        AT_NULL,
        0,
    ];

    // handle envs and args
    let mut env_vec = vec![];
    let mut arg_vec = vec![];
    let mut argc = 0;

    let mut envp = envp as *const usize;
    unsafe {
        while *envp != 0 {
            env_vec.push(*envp);
            envp = envp.add(1);
        }
        env_vec.push(0);
    }

    let mut argv = argv as *const usize;
    unsafe {
        while *argv != 0 {
            arg_vec.push(*argv);
            argv = argv.add(1);
            argc += 1;
        }
        arg_vec.push(0);
    }

    // push
    stack.push(&auxv, 16);
    stack.push(&env_vec, 8);
    stack.push(&arg_vec, 8);
    let sp = stack.push(&[argc], 8);

    // try run
    debug!(
        "sys_execve: sp is 0x{sp:x}, run at 0x{entry:x}, then jump to 0x{:x} ",
        prog.entry
    );

    set_sp_and_jmp(sp, entry);
}

fn set_sp_and_jmp(sp: usize, entry: usize) -> ! {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::arch::asm!("
         mov sp, {}
         br {}
     ",
        in(reg)sp,
        in(reg)entry,
        );
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("
         mov rsp, {}
         jmp {}
     ",
        in(reg)sp,
        in(reg)entry,
        );
    }
    unreachable!("sys_execve: unknown arch, sp 0x{sp:x}, entry 0x{entry:x}");
}

/// for AT_RANDOM
fn rand() -> [i64; 2] {
    unsafe { [sys_random(), sys_random()] }
}

fn platform() -> usize {
    #[cfg(target_arch = "aarch64")]
    const PLATFORM_STRING: &[u8] = b"aarch64\0";
    #[cfg(target_arch = "x86_64")]
    const PLATFORM_STRING: &[u8] = b"x86_64\0";
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    const PLATFORM_STRING: &[u8] = b"unknown\0";

    PLATFORM_STRING.as_ptr() as usize
}
