# tiny-docker: Write a Tiny Docker in 100 Lines of C/C++ and Rust

This repository implements a tiny-docker separately in C/C++ and Rust, inspired by [Containers From Scratch • Liz Rice • GOTO 2018](https://www.youtube.com/watch?app=desktop&v=8fi7uSYlOdc&feature=youtu.be), which illustrates the idea of Docker in Golang. This project can only run on GNU/Linux.

First, we need to generate a Linux filesystem for tiny-docker based on your current system's filesystem:
```sh
cd docker-fs
sudo tar -c -p -f docker-fs.tar --exclude=/home --one-file-system /
sudo tar xf docker-fs.tar
```
The above instructions exclude the `/home` directory and the `--one-file-system` option ensures it does not cross filesystem boundaries. The `-p` flag preserves the file permissions in the archive and `/` specifies the root directory to be archived.

Now the filesystem is ready. Enter `docker` or `dockerr` and run these commands:
```sh
make 
sudo make run
```
You will see output like this:
```txt
Parent is running /bin/bash pid 1004025
Child is running /bin/bash pid 1
root@TinyDocker:/# exit
```
which illustrates that the hostname is isolated.

Then run `ls`:
```txt
root@TinyDocker:/# ls
bin  boot  dev  etc  lib  lib32  lib64  libx32  media  mnt  opt  proc  root  run  sbin  srv  sys  tmp  docker-fs.tar  usr  var
```
which illustrates that the filesystem is isolated.

Finally, run `ps aux`:
```txt
root@TinyDocker:/# ps aux
USER         PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND
root           1  0.0  0.0   3256  1108 ?        S    21:51   0:00 target/release/dockerr run /bin/bash
root           2  0.0  0.0   4248  3584 ?        S    21:51   0:00 /bin/bash
root          10  0.0  0.0   5900  3012 ?        R+   21:51   0:00 ps aux
```
which illustrates that the pid namespace is isolated.