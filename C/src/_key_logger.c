// Copyleft Creative Commons Attribution-Share Alike 4.0 International (CC BY-SA 4.0)
// License holder: jtyrrell
// Date: 07-13-2012, 07:54 AM 
// Original source: https://www.linuxquestions.org/questions/linux-general-1/reading-and-writing-to-the-linux-keyboard-buffer-4175416506/

// Purpose: It writes KEY_(...) to input buffer, it mimicks the press of
// a key, it then scans the input buffer to retrieve the scan code and writes
// it to a semi-random file.
// Requirement: Use root.

#include <stdio.h>
#include <fcntl.h> // File control options
#include <linux/input.h> 
#include <unistd.h> // open(), write(), close()
#include <stdlib.h>

#define EV_PRESSED 1
#define EV_RELEASED 0
#define EV_REPEAT 2

int main()
{
    printf("Starting the keyboard buffer writer/reader \n");
    int fd = 0;

    // This is the keyboard device as identified using:
    // $ cat /proc/bus/input/devices
    char *device = "/dev/input/event0"; 

    /* Write a key to the keyboard buffer,
     * IDK what "> 0" does here? */
    if ((fd = open(device, O_RDWR)) > 0)
    {
        struct input_event event;

        // Press a key 
        event.type = EV_KEY;
        event.value = EV_PRESSED;
        // https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
        event.code = KEY_0;

        write(fd, &event, sizeof(struct input_event));

        // Release the key
        event.value = EV_RELEASED;
        event.code = KEY_0;

        write(fd, &event, sizeof(struct input_event));
        close(fd);

        printf("The keyboard code is: %d \n", KEY_0); 
    }

    /* Read the key back from the keyboard buffer */
    int fd1 = 0;
    if ((fd1 = open(device, O_RDONLY)) > 0)
    {
        unsigned int scan_code = 0;
        struct input_event event;

        if(event.type != EV_KEY)
        {
            // Keyboard events are always of type EV_KEY
            return 0; 
        }

        if(event.value == EV_RELEASED)
        {
            scan_code = event.code;
            printf("Read back scan_code is: %u\n", scan_code);

            // This isn't really needed, but it's a good lesson regardless
            char template[] = "/tmp/KEY_LOGGER_XXXXXX";
            mkstemp(template);

            FILE *fptr;
            fptr = fopen(template, "a");
            fprintf(fptr, "%u", scan_code, "\n");
        }

        close(fd1);
    }
}
