use num_enum::TryFromPrimitive;

// TODO: syscall id are architecture-dependent
#[allow(clippy::upper_case_acronyms)]
#[allow(non_camel_case_types)]
#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
pub enum SyscallId {
    INVALID = 999,
    #[cfg(feature = "fs")]
    GETCWD = 17,
    #[cfg(feature = "epoll")]
    EPOLL_CREATE1 = 20,
    #[cfg(feature = "epoll")]
    EPOLL_CTL = 21,
    #[cfg(feature = "epoll")]
    EPOLL_PWAIT = 22,
    #[cfg(feature = "fd")]
    DUP = 23,
    #[cfg(feature = "fd")]
    DUP3 = 24,
    #[cfg(feature = "fd")]
    FCNTL = 25,
    #[cfg(feature = "fd")]
    IOCTL = 29,
    #[cfg(feature = "fs")]
    MKDIRAT = 34,
    #[cfg(feature = "fs")]
    UNLINKAT = 35,
    #[cfg(feature = "fs")]
    RENAMEAT = 38,
    #[cfg(feature = "fs")]
    FACCESSAT = 48,
    #[cfg(feature = "fs")]
    CHDIR = 49,
    #[cfg(feature = "fs")]
    FCHOWNAT = 54,
    #[cfg(feature = "fs")]
    OPENAT = 56,
    #[cfg(feature = "fd")]
    CLOSE = 57,
    #[cfg(feature = "pipe")]
    PIPE2 = 59,
    #[cfg(feature = "fs")]
    GETDENTS64 = 61,
    #[cfg(feature = "fs")]
    LSEEK = 62,
    READ = 63,
    WRITE = 64,
    #[cfg(feature = "fd")]
    READV = 65,
    #[cfg(feature = "fd")]
    WRITEV = 66,
    #[cfg(feature = "fs")]
    PREAD64 = 67,
    #[cfg(feature = "fs")]
    PWRITE64 = 68,
    #[cfg(feature = "fs")]
    PREADV = 69,
    #[cfg(feature = "select")]
    PSELECT6 = 72,
    #[cfg(feature = "poll")]
    PPOLL = 73,
    #[cfg(feature = "fs")]
    READLINKAT = 78,
    #[cfg(feature = "fs")]
    NEWFSTATAT = 79,
    #[cfg(feature = "fs")]
    FSTAT = 80,
    #[cfg(feature = "fs")]
    FSYNC = 82,
    #[cfg(feature = "fs")]
    FDATASYNC = 83,
    TIMERFD_CREATE = 85,
    CAP_GET = 90,
    EXIT = 93,
    #[cfg(feature = "multitask")]
    SET_TID_ADDRESS = 96,
    #[cfg(feature = "multitask")]
    FUTEX = 98,
    NANO_SLEEP = 101,
    CLOCK_SETTIME = 112,
    CLOCK_GETTIME = 113,
    CLOCK_GETRES = 114,
    // SCHED_GETAFFINITY = 123,
    SCHED_YIELD = 124,
    #[cfg(feature = "signal")]
    KILL = 129,
    #[cfg(feature = "signal")]
    TKILL = 130,
    #[cfg(feature = "signal")]
    SIGALTSTACK = 132,
    #[cfg(feature = "signal")]
    RT_SIGACTION = 134,
    #[cfg(feature = "signal")]
    RT_SIGPROCMASK = 135,
    SETGID = 144,
    SETUID = 146,
    TIMES = 153,
    SETPGID = 154,
    GETPGID = 155,
    UNAME = 160,
    GETRLIMIT = 163,
    SETRLIMIT = 164,
    UMASK = 166,
    #[cfg(feature = "multitask")]
    GETPID = 172,
    GETPPID = 173,
    GETUID = 174,
    GETEUID = 175,
    GETGID = 176,
    GETEGID = 177,
    GETTID = 178,
    SYSINFO = 179,
    #[cfg(feature = "net")]
    SOCKET = 198,
    #[cfg(feature = "net")]
    SOCKETPAIR = 199,
    #[cfg(feature = "net")]
    BIND = 200,
    #[cfg(feature = "net")]
    LISTEN = 201,
    #[cfg(feature = "net")]
    ACCEPT = 202,
    #[cfg(feature = "net")]
    CONNECT = 203,
    #[cfg(feature = "net")]
    GETSOCKNAME = 204,
    #[cfg(feature = "net")]
    GETPEERNAME = 205,
    #[cfg(feature = "net")]
    SENDTO = 206,
    #[cfg(feature = "net")]
    RECVFROM = 207,
    #[cfg(feature = "net")]
    SETSOCKOPT = 208,
    #[cfg(feature = "net")]
    SHUTDOWN = 210,
    #[cfg(feature = "net")]
    SENDMSG = 211,
    #[cfg(feature = "alloc")]
    MUNMAP = 215,
    #[cfg(feature = "alloc")]
    MREMAP = 216,
    #[cfg(feature = "multitask")]
    CLONE = 220,
    #[cfg(feature = "fs")]
    EXECVE = 221,
    #[cfg(feature = "alloc")]
    MMAP = 222,
    #[cfg(feature = "alloc")]
    MPROTECT = 226,
    #[cfg(feature = "alloc")]
    MSYNC = 227,
    #[cfg(feature = "alloc")]
    MADVISE = 233,
    PRLIMIT64 = 261,
    GETRANDOM = 278,
}
