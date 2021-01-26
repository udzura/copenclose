copenclose(8)
=============

Tracking open/close operations for containers; using either cgroup v2 or namespace. 

### Supported syscalls

- open
- openat
- accept
- accept4
- connect
- close

## Usage

```console
copenclose 0.1.0

USAGE:
    copenclose [FLAGS]

FLAGS:
    -h, --help              Prints help information
    -I, --ignore-host-ns    
    -C, --use-cgv2          
    -V, --version           Prints version information
```

Without `-C`, copenclose uses UTS namespace to determine containers.

```console
$ sudo ./target/debug/copenclose -I
TIME                 HOSTNAME         PID    UID    GID    SYSCALL  COMM
2021-01-19T07:35:10Z 6643ca763e3e     107148 100000 100000 accept4  httpd
2021-01-19T07:35:10Z 6643ca763e3e     107147 100000 100000 accept4  httpd
2021-01-19T07:35:10Z 6643ca763e3e     107146 100000 100000 accept4  httpd
2021-01-19T07:35:10Z 6643ca763e3e     107148 100000 100000 openat   httpd
2021-01-19T07:35:10Z 6643ca763e3e     107148 100000 100000 close    httpd
2021-01-19T07:35:10Z 6643ca763e3e     107148 100000 100000 close    httpd
2021-01-19T07:35:11Z f75f4ffd7f1d     106962 100000 100000 accept4  httpd
2021-01-19T07:35:11Z f75f4ffd7f1d     106963 100000 100000 accept4  httpd
2021-01-19T07:35:11Z f75f4ffd7f1d     106961 100000 100000 accept4  httpd
2021-01-19T07:35:11Z f75f4ffd7f1d     106963 100000 100000 openat   httpd
2021-01-19T07:35:11Z f75f4ffd7f1d     106963 100000 100000 close    httpd
2021-01-19T07:35:11Z f75f4ffd7f1d     106963 100000 100000 close    httpd
2021-01-19T07:35:14Z 1e012607c8bf     106767 100000 100000 accept4  httpd
2021-01-19T07:35:14Z 1e012607c8bf     106766 100000 100000 accept4  httpd
2021-01-19T07:35:14Z 1e012607c8bf     106767 100000 100000 openat   httpd
2021-01-19T07:35:14Z 1e012607c8bf     106767 100000 100000 close    httpd
2021-01-19T07:35:14Z 1e012607c8bf     106767 100000 100000 close    httpd
```

With `-C`, the tool uses cgroup v2 id. Is is supported by runtimes such as podman/crun.

```console
$ sudo ./target/debug/copenclose -I -C
TIME                 CGROUPID PID    UID    GID    SYSCALL  COMM
2021-01-19T07:36:05Z     3333 106765 100000 100000 accept4  httpd
2021-01-19T07:36:05Z     3333 106765 100000 100000 openat   httpd
2021-01-19T07:36:05Z     3333 106765 100000 100000 openat   httpd
2021-01-19T07:36:05Z     3333 106765 100000 100000 close    httpd
2021-01-19T07:36:05Z     3333 106765 100000 100000 close    httpd
2021-01-19T07:36:05Z     3333 106765 100000 100000 close    httpd
2021-01-19T07:36:06Z     3348 106962 100000 100000 accept4  httpd
2021-01-19T07:36:06Z     3348 106962 100000 100000 openat   httpd
2021-01-19T07:36:06Z     3348 106962 100000 100000 close    httpd
2021-01-19T07:36:06Z     3348 106962 100000 100000 close    httpd
2021-01-19T07:36:06Z     3363 107148 100000 100000 accept4  httpd
2021-01-19T07:36:06Z     3363 107146 100000 100000 accept4  httpd
2021-01-19T07:36:06Z     3363 107148 100000 100000 openat   httpd
2021-01-19T07:36:06Z     3363 107148 100000 100000 close    httpd
2021-01-19T07:36:06Z     3363 107148 100000 100000 close    httpd
```

With `-I`, the tool ignores events occured in the current host namespace.

### Use via Docker

* [Dockerhub](https://hub.docker.com/r/udzura/copenclose/tags?page=1&ordering=last_updated)

```console
$ sudo docker pull udzura/copenclose:latest
$ sudo docker run -ti --privileged udzura/copenclose:latest
TIME                 HOSTNAME         PID    UID    GID    SYSCALL  COMM
2021-01-26T10:06:14Z ubuntu-groovy    829    0      0      connect  snapd
2021-01-26T10:06:14Z ubuntu-groovy    829    0      0      close    snapd
2021-01-26T10:06:14Z 5c84c9955c20     31297  0      0      openat   copenclose
2021-01-26T10:06:14Z ubuntu-groovy    656    0      0      openat   multipathd
2021-01-26T10:06:14Z ubuntu-groovy    656    0      0      close    multipathd
2021-01-26T10:06:14Z ubuntu-groovy    656    0      0      openat   multipathd
2021-01-26T10:06:14Z ubuntu-groovy    656    0      0      close    multipathd
2021-01-26T10:06:14Z ubuntu-groovy    23132  0      0      openat   systemd-journal
...
```

* Host Requirements:
  * Kernel >= 5.5
  * Kernel with `CONFIG_DEBUG_INFO_BTF=y`
