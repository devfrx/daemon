"""SP-7 / S2 -- the worker half of the round trip worker -> relay -> page.

One JSON line per result on stdout: {"t_capture_ms": <wall-clock ms when the frame was read>,
"hands": [[[x, y], ... 21 points ...], ...]}, with x and y as INTEGER pixels of a 640x480 frame:
integers on the wire is the rule of the design (§2.3), and the worker does the scaling. The relay
spawns this script as a child process and reads its stdout, which is the topology of the core and
a worker. Stops when stdout is closed by the relay, or on Ctrl-C.

Usage: py -3.10 s2_worker.py hand_landmarker.task
"""
import json
import sys
import threading
import time

import cv2
import mediapipe as mp

BaseOptions = mp.tasks.BaseOptions
HandLandmarker = mp.tasks.vision.HandLandmarker
HandLandmarkerOptions = mp.tasks.vision.HandLandmarkerOptions
VisionRunningMode = mp.tasks.vision.RunningMode

WIDTH, HEIGHT = 640, 480


def main():
    model = sys.argv[1] if len(sys.argv) > 1 else "hand_landmarker.task"
    captured_at = {}  # timestamp_ms -> wall-clock ms at capture
    stop = threading.Event()

    def on_result(result, output_image, timestamp_ms):
        t_capture = captured_at.pop(timestamp_ms, None)
        if t_capture is None:
            return
        hands = [
            [[int(round(lm.x * WIDTH)), int(round(lm.y * HEIGHT))] for lm in hand]
            for hand in result.hand_landmarks
        ]
        try:
            sys.stdout.write(json.dumps({"t_capture_ms": t_capture, "hands": hands}) + "\n")
            sys.stdout.flush()
        except (BrokenPipeError, OSError):
            stop.set()  # the relay is gone: the worker has nobody to talk to

    options = HandLandmarkerOptions(
        base_options=BaseOptions(model_asset_path=model),
        running_mode=VisionRunningMode.LIVE_STREAM,
        num_hands=2,
        result_callback=on_result,
    )
    cap = cv2.VideoCapture(0)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, WIDTH)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, HEIGHT)
    last_ts = -1
    start = time.perf_counter_ns()
    with HandLandmarker.create_from_options(options) as landmarker:
        while not stop.is_set():
            ok, frame = cap.read()
            if not ok:
                break
            wall_ms = time.time_ns() // 1_000_000
            rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb)
            ts = max(last_ts + 1, (time.perf_counter_ns() - start) // 1_000_000)
            last_ts = ts
            captured_at[ts] = wall_ms
            landmarker.detect_async(image, ts)
    cap.release()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
