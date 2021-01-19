#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

struct event {
  pid_t pid;
  u32 tid;
  uid_t uid;
  gid_t gid;
  u32 syscall_nr;
  char comm[32];
  char hostname[32];
  __u64 cgid;
};

const volatile u32 pid_self = 0;
const volatile bool ignore_host_ns = false;
const volatile bool use_cgroup_v2 = false;

struct {
  __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
  __uint(key_size, sizeof(u32));
  __uint(value_size, sizeof(u32));
} events SEC(".maps");

SEC("tracepoint/raw_syscalls/sys_enter")
int sys_enter(struct trace_event_raw_sys_enter *args)
{
  struct event event = {0};

  u64 id = bpf_get_current_pid_tgid();
  pid_t pid = id >> 32;
  u32 tid = id;
  u64 ugid = bpf_get_current_uid_gid();
  uid_t uid = ugid;
  gid_t gid = ugid >> 32;

  struct task_struct *task = (void *)bpf_get_current_task();
  u64 inum;
  inum = BPF_CORE_READ(task,
                       nsproxy, pid_ns_for_children, ns.inum);
  // 0xEFFFFFFCU is initial host namespace id
  if( inum == 0xEFFFFFFCU && ignore_host_ns )
    return 0;

  // return if process is self
  if( pid == pid_self )
    return 0;

  // in 86_64:
  if( args->id != 2   && // open
      args->id != 257 && // openat
      args->id != 3   && // close
      args->id != 42  && // connect
      args->id != 43  && // accept
      args->id != 288    // accept4
      )
    return 0;

  event.pid = pid;
  event.tid = tid;
  event.uid = uid;
  event.gid = gid;
  bpf_get_current_comm(&event.comm, sizeof(event.comm));
  event.syscall_nr = args->id;

  if( use_cgroup_v2 ) {
    event.cgid = bpf_get_current_cgroup_id();
  } else {
    BPF_CORE_READ_STR_INTO(&event.hostname,
                           task,
                           nsproxy, uts_ns, name.nodename);
  }

  bpf_perf_event_output(args, &events, BPF_F_CURRENT_CPU,
                        &event, sizeof(event));
  return 0;
}

char LICENSE[] SEC("license") = "GPL";
