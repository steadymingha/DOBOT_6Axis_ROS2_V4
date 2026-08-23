"""Minimal joydev (/dev/input/jsN) gamepad reader.

Uses the kernel's joydev API, which already normalizes whatever HID report
format the pad speaks into a flat axis/button index space -- no per-pad-model
parsing needed. Axis/button *numbering* still varies by device, so run this
file directly (--js-test) to find out which index is which before wiring
jog_real.py to specific indices.

    JOG_JS_DEV=/dev/input/js0 python3 gamepad.py --js-test
"""
import os
import struct
import sys
import threading
import time

JS_EVENT_FMT = 'IhBB'   # time_ms, value(-32767..32767), type, number
JS_EVENT_SIZE = struct.calcsize(JS_EVENT_FMT)
JS_TYPE_BUTTON = 0x01
JS_TYPE_AXIS = 0x02
JS_TYPE_INIT = 0x80

DEADZONE = 0.15


class Gamepad:
    """Background reader; .axes[i] in [-1,1], .buttons[i] in {0,1}."""

    def __init__(self, dev=None):
        self.dev = dev or os.environ.get('JOG_JS_DEV', '/dev/input/js0')
        self.axes = {}
        self.buttons = {}
        self._fh = None
        self._thread = None
        self._running = False

    def open(self):
        try:
            self._fh = open(self.dev, 'rb')
        except OSError as e:
            print(f'[gamepad] {self.dev} unavailable ({e})')
            return False
        self._running = True
        self._thread = threading.Thread(target=self._reader, daemon=True)
        self._thread.start()
        return True

    def _reader(self):
        while self._running:
            evt = self._fh.read(JS_EVENT_SIZE)
            if not evt:
                break
            _, value, typ, number = struct.unpack(JS_EVENT_FMT, evt)
            kind = typ & ~JS_TYPE_INIT
            if kind == JS_TYPE_AXIS:
                self.axes[number] = value / 32767.0
            elif kind == JS_TYPE_BUTTON:
                self.buttons[number] = value

    def axis(self, number, deadzone=DEADZONE):
        v = self.axes.get(number, 0.0)
        return v if abs(v) > deadzone else 0.0

    def button(self, number):
        return bool(self.buttons.get(number, 0))

    def close(self):
        # Don't call self._fh.close() here: the reader thread is almost always
        # blocked inside self._fh.read() (no fresh pad activity at shutdown time),
        # and close() on a buffered file blocks waiting for the same internal lock
        # that read() is holding -- classic close-vs-blocking-read deadlock, hangs
        # the whole process forever. _reader is a daemon thread; just stop caring
        # about it and let process exit reclaim the fd.
        self._running = False


def js_test(seconds):
    pad = Gamepad()
    if not pad.open():
        return
    print(f'[gamepad] reading {pad.dev} for {seconds}s -- move every stick/trigger, '
          f'press every button now')
    seen_axes, seen_buttons = {}, {}
    t_end = time.time() + seconds
    while time.time() < t_end:
        for n, v in list(pad.axes.items()):
            if seen_axes.get(n) != round(v, 2):
                seen_axes[n] = round(v, 2)
                print(f'  AXIS {n:2d}: {v:+.2f}')
        for n, v in list(pad.buttons.items()):
            if seen_buttons.get(n) != v:
                seen_buttons[n] = v
                print(f'  BUTTON {n:2d}: {v}')
        time.sleep(0.05)
    pad.close()
    print('[gamepad] done. axis range near 0 at rest = stick; '
          'axis resting at -1.00 = analog trigger unpressed.')


if __name__ == '__main__':
    secs = 12
    if '--js-test' in sys.argv:
        i = sys.argv.index('--js-test')
        if len(sys.argv) > i + 1 and sys.argv[i + 1].isdigit():
            secs = int(sys.argv[i + 1])
    js_test(secs)
