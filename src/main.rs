use core::time::Duration;
use std::str;
use std::collections::HashMap;

use chrono::{Local, SecondsFormat};
use anyhow::Result;
use libbpf_rs::PerfBufferBuilder;
use plain::Plain;
use structopt::StructOpt;

mod bpf;
use bpf::*;

#[derive(Debug, StructOpt)]
struct Command {
    #[structopt(short = "C", long = "use-cgv2")]
    use_cgroup_v2: bool,
    #[structopt(short = "I", long = "ignore-host-ns")]
    ignore_host_ns: bool,
}

#[repr(C)]
#[derive(Default)]
#[derive(Debug)]
struct Event {
    pub pid: u32,
    pub tid: u32,
    pub uid: u32,
    pub gid: u32,
    pub syscall_nr: u32,
    pub comm: [u8; 32],
    pub hostname: [u8; 32],
    pub cgid: u64,
}
unsafe impl Plain for Event {}

fn to_syscall_name(nr: u32) -> &'static str {
    let mut map = HashMap::new();
    map.insert(2, "open");
    map.insert(257, "openat");
    map.insert(3, "close");
    map.insert(42, "connect");
    map.insert(43, "accept");
    map.insert(288, "accept4");

    if let Some(val) = map.get(&nr) {
        return val;
    }
    "unknown"
}

fn handle_event_hn(_cpu: i32, data: &[u8]) {
    let now = Local::now();
    let mut event: Event = Event::default();
    plain::copy_from_bytes(&mut event, data).expect("Data buffer was too short or invalid");
    let comm = str::from_utf8(&event.comm).unwrap().trim_end_matches('\0');
    let hostname = str::from_utf8(&event.hostname).unwrap().trim_end_matches('\0');
    let syscall = to_syscall_name(event.syscall_nr);

    println!("{:20} {:16} {:<6} {:<6} {:<6} {:8} {}",
             now.to_rfc3339_opts(SecondsFormat::Secs, true),
             hostname,
             event.pid,
             event.uid,
             event.gid,
             syscall,
             comm );
}

fn handle_event_cg(_cpu: i32, data: &[u8]) {
    let now = Local::now();
    let mut event: Event = Event::default();
    plain::copy_from_bytes(&mut event, data).expect("Data buffer was too short or invalid");
    let comm = str::from_utf8(&event.comm).unwrap().trim_end_matches('\0');
    let syscall = to_syscall_name(event.syscall_nr);

    println!("{:20} {:8} {:<6} {:<6} {:<6} {:8} {}",
             now.to_rfc3339_opts(SecondsFormat::Secs, true),
             event.cgid,
             event.pid,
             event.uid,
             event.gid,
             syscall,
             comm );
}

fn handle_lost_events(cpu: i32, count: u64) {
    eprintln!("[!] Lost {} events on CPU {}", count, cpu);
}

fn main() -> Result<()> {
    let opts: Command = Command::from_args();

    let mut skel_builder: CopencloseSkelBuilder = CopencloseSkelBuilder::default();
    let mut open_skel: OpenCopencloseSkel = skel_builder.open()?;

    open_skel.rodata().ignore_host_ns = if opts.ignore_host_ns { 1 } else { 0 };
    open_skel.rodata().use_cgroup_v2 = if opts.use_cgroup_v2 { 1 } else { 0 };
    open_skel.rodata().pid_self = std::process::id();
    let mut skel = open_skel.load()?;

    if opts.use_cgroup_v2 {
        println!("{:20} {:8} {:6} {:6} {:6} {:8} COMM",
                 "TIME", "CGROUPID", "PID", "UID", "GID", "SYSCALL");
    } else {
        println!("{:20} {:16} {:6} {:6} {:6} {:8} COMM",
                 "TIME", "HOSTNAME", "PID", "UID", "GID", "SYSCALL");
    }
    skel.attach()?;

    let cb = if opts.use_cgroup_v2 { handle_event_cg } else { handle_event_hn };
    let perf = PerfBufferBuilder::new(skel.maps().events())
        .sample_cb(cb)
        .lost_cb(handle_lost_events)
        .build()?;

    loop {
        perf.poll(Duration::from_millis(100))?;
    }
}
