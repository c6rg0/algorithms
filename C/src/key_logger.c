// Purpose: It scans the input buffer to retrieve the scan code and writes
// it to a file (not forever).

// Requirement: Use root and don't use wayland: if you are, drop to tty (ctrl+alt+2) 

#include <stdio.h>
#include <fcntl.h> // File control options
#include <linux/input.h> 
#include <unistd.h> // open(), write(), close()
#include <stdlib.h>
#include <stddef.h>
#include <sys/ioctl.h>

int main()
{
    printf("> Starting the keyboard buffer reader \n");

    int fd = open("/dev/input/event0", O_RDONLY);
    if (fd == -1) {
        perror("open");
        return 1;
    }

    ioctl(fd, EVIOCGRAB, 1); // All key events go to this program
    struct input_event event;

    int i = 1;
    while (i < 67)
    {
        ssize_t res = read(fd, &event, sizeof(event));
        printf("type=%d code=%d value=%d\n", event.type, event.code, event.value);

        if (res == sizeof(event) && event.type == EV_KEY && event.value == 1)
        {
            printf("Read back scan_code is: %u\n", event.code);

            FILE *fptr;
            fptr = fopen("/tmp/KEY_LOGGER", "a");
            fprintf(fptr, "%u\n", event.code);
            fclose(fptr);
            i++;
        }
    }

    close(fd);
    return 0;
}
