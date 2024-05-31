#include <iostream>
#include <string>
#include <cstring>
#include <sched.h>
#include <unistd.h>
#include <sys/wait.h>
#include <sys/mount.h>

using namespace std;

static void run(int argc, char **argv);
static string cmd(int argc, char **argv);
static void run_child(int argc, char **argv);

const char *CHILD_HOSTNAME = "TinyDocker";

int main(int argc, char **argv) {
    if (argc < 3) {
        cerr << "Too few arguments" << endl;
        exit(-1);
    }

    if (!strcmp(argv[1], "run")) {
        run(argc - 2, &argv[2]);
    }
}

static void run(int argc, char **argv) {
    cout << "Parent is running " << cmd(argc, argv) << "pid " << getpid() << endl;    

    if (unshare(CLONE_NEWPID) < 0) {
        cerr << "Failed to unshare PID namespace" << endl;
        exit(-1);
    }

    pid_t child_pid = fork();
    
    if (child_pid < 0) {
        cerr << "Failed to fork child" << endl;
        return;
    }

    if (child_pid) {
        if(waitpid(child_pid, NULL, 0) < 0) {
            cerr << "Failed to wait for child" << endl;
        }
    } else {        
        run_child(argc, argv);
    }
}

static string cmd(int argc, char **argv) {
    string cmd = "";
    for (int i = 0; i < argc; i++) {
        cmd.append(argv[i] + string(" "));
    }
    return cmd;
}

static void run_child(int argc, char **argv) {
    cout << "Child is running " << cmd(argc, argv) << "pid " << getpid() << endl;    

    int flags = CLONE_NEWUTS | CLONE_NEWNS;

    if (unshare(flags) < 0) {
        cerr << "Failed to unshare in child" << endl;
        exit(-1);
    }

    if (mount(NULL, "/", NULL, MS_SLAVE | MS_REC, NULL) < 0) {
        cerr << "Failed to mount /" << endl;
        exit(-1);
    } 

    if (chroot("../docker-fs") < 0) {
        cerr << "Failed to chroot" << endl;
        exit(-1);
    }

    if (chdir("/") < 0) {
        cerr << "Failed to chdir to /" << endl;
        exit(-1);
    }

    if (mount("proc", "proc", "proc", 0, NULL) < 0) {
        cerr << "Failed to mount /proc" << endl;
        exit(-1);
    } 

    if (sethostname(CHILD_HOSTNAME, strlen(CHILD_HOSTNAME)) < 0) {
        cerr << "Failed to change hostname" << endl;
        exit(-1);
    }

    if (execvp(argv[0], argv)) {
        cerr << "Failed to exec" << endl;
    }
}
