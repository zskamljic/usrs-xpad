#include <linux/uinput.h>
#include <memory.h>
#include <stdbool.h>
#include <unistd.h>

int get_code(char value);

void setup_device(int file, const char *name, short vendor, short product) {
    struct uinput_setup usetup;

    ioctl(file, UI_SET_EVBIT, EV_KEY);
    ioctl(file, UI_SET_KEYBIT, KEY_SPACE);
    ioctl(file, UI_SET_KEYBIT, KEY_D);
    ioctl(file, UI_SET_KEYBIT, KEY_E);
    ioctl(file, UI_SET_KEYBIT, KEY_N);
    ioctl(file, UI_SET_KEYBIT, KEY_S);
    ioctl(file, UI_SET_KEYBIT, KEY_U);

    memset(&usetup, 0, sizeof(usetup));
    usetup.id.bustype = BUS_USB;
    usetup.id.vendor = vendor;
    usetup.id.product = product;
    strcpy(usetup.name, name);

    ioctl(file, UI_DEV_SETUP, &usetup);
    ioctl(file, UI_DEV_CREATE);
}

void emit(int file, int type, int code, int val) {
    struct input_event event;

    event.type = type;
    event.code = code;
    event.value = val;
    /* timestamp values below are ignored */
    event.time.tv_sec = 0;
    event.time.tv_usec = 0;

    write(file, &event, sizeof(event));
}

void set_key(int file, char key, bool pressed) {
    int keycode = get_code(key);
    emit(file, EV_KEY, keycode, pressed);
    emit(file, EV_SYN, SYN_REPORT, 0);
}

void close_and_destroy(int file) {
    ioctl(file, UI_DEV_DESTROY);
    close(file);
}

int get_code(char value) {
    switch (value) {
        case 'D':
            return KEY_D;
        case 'E':
            return KEY_E;
        case 'N':
            return KEY_N;
        case 'S':
            return KEY_S;
        case 'U':
            return KEY_U;
        default:
            return KEY_SPACE;
    }
}