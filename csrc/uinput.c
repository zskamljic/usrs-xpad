#include <limits.h>
#include <linux/uinput.h>
#include <memory.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdio.h>

enum Key {
    A, B, X, Y, LB, RB, BACK, SELECT, UP, DOWN, LEFT, RIGHT, THUMB_LEFT, THUMB_RIGHT, XBOX
};

int get_code(int value);

void setup_abs(int file, unsigned channel, int min, int max);

void setup_device(int file, const char *name, short vendor, short product) {
    struct uinput_setup usetup;

    // Enable standard keys
    ioctl(file, UI_SET_EVBIT, EV_KEY);
    ioctl(file, UI_SET_KEYBIT, BTN_A);
    ioctl(file, UI_SET_KEYBIT, BTN_B);
    ioctl(file, UI_SET_KEYBIT, BTN_X);
    ioctl(file, UI_SET_KEYBIT, BTN_Y);
    ioctl(file, UI_SET_KEYBIT, BTN_TL);
    ioctl(file, UI_SET_KEYBIT, BTN_TR);
    ioctl(file, UI_SET_KEYBIT, BTN_BACK);
    ioctl(file, UI_SET_KEYBIT, BTN_START);
    ioctl(file, UI_SET_KEYBIT, BTN_SELECT);
    ioctl(file, UI_SET_KEYBIT, BTN_TRIGGER_HAPPY1);
    ioctl(file, UI_SET_KEYBIT, BTN_TRIGGER_HAPPY2);
    ioctl(file, UI_SET_KEYBIT, BTN_TRIGGER_HAPPY3);
    ioctl(file, UI_SET_KEYBIT, BTN_TRIGGER_HAPPY4);
    ioctl(file, UI_SET_KEYBIT, BTN_THUMBL);
    ioctl(file, UI_SET_KEYBIT, BTN_THUMBR);

    // Enable joysticks
    ioctl(file, UI_SET_EVBIT, EV_ABS);
    setup_abs(file, ABS_X, SHRT_MIN, SHRT_MAX);
    setup_abs(file, ABS_Y, SHRT_MIN, SHRT_MAX);
    setup_abs(file, ABS_Z, 0, 1024);
    setup_abs(file, ABS_RX, SHRT_MIN, SHRT_MAX);
    setup_abs(file, ABS_RY, SHRT_MIN, SHRT_MAX);
    setup_abs(file, ABS_RZ, 0, 1024);

    memset(&usetup, 0, sizeof(usetup));
    usetup.id.bustype = BUS_USB;
    usetup.id.vendor = vendor;
    usetup.id.product = product;
    usetup.id.version = 2;
    strcpy(usetup.name, name);

    ioctl(file, UI_DEV_SETUP, &usetup);
    ioctl(file, UI_DEV_CREATE);
}

void emit(int file, int type, int code, int val) {
    struct input_event event = {
        .type = type,
        .code = code,
        .value = val,
        /* timestamp values below are ignored */
        .time = {
                .tv_sec = 0,
                .tv_usec = 0,
        },
    };

    write(file, &event, sizeof(event));
}

void set_key(int file, int key, bool pressed) {
    int keycode = get_code(key);
    emit(file, EV_KEY, keycode, pressed);
    emit(file, EV_SYN, SYN_REPORT, 0);
}

void set_axis(int file, int side, short x, short y, unsigned short z) {
    emit(file, EV_ABS, side ? ABS_RX : ABS_X, x);
    emit(file, EV_ABS, side ? ABS_RY : ABS_Y, -y);
    emit(file, EV_ABS, side ? ABS_RZ : ABS_Z, z);
    emit(file, EV_SYN, SYN_REPORT, 0);
}

void close_and_destroy(int file) {
    ioctl(file, UI_DEV_DESTROY);
    close(file);
}

int get_code(int value) {
    switch (value) {
        case A:
            return BTN_A;
        case B:
            return BTN_B;
        case X:
            return BTN_X;
        case Y:
            return BTN_Y;
        case LB:
            return BTN_TL;
        case RB:
            return BTN_TR;
        case BACK:
            return BTN_BACK;
        case SELECT:
            return BTN_SELECT;
        case UP:
            return BTN_TRIGGER_HAPPY3;
        case DOWN:
            return BTN_TRIGGER_HAPPY4;
        case LEFT:
            return BTN_TRIGGER_HAPPY1;
        case RIGHT:
            return BTN_TRIGGER_HAPPY2;
        case XBOX:
            return BTN_START;
        case THUMB_LEFT:
            return BTN_THUMBL;
        case THUMB_RIGHT:
            return BTN_THUMBR;
        default:
            return KEY_SPACE;
    }
}

void setup_abs(int file, unsigned chan, int min, int max) {
    ioctl(file, UI_SET_ABSBIT, chan);
    struct uinput_abs_setup setup = {
            .code = chan,
            .absinfo = {
                    .minimum = min,
                    .maximum = max,
            },
    };
    ioctl(file, UI_ABS_SETUP, &setup);
}