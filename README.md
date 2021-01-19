copenclose(8)
=============

Tracking open/close operations for containers; using either cgroup v2 or namespace. 

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

With `-C`, the tool uses cgroup v2 id. Is is supported by runtimes such as podman/crun.

With `-I`, the tool ignores events occured in the current host.

## Use inside containers

// TBD
