"""SP-7 / S1 -- does MediaPipe Hand Landmarker on the CPU hold 30 Hz on this machine?

The criteria live in PROTOCOLLO.md and were frozen before this file was written. The latency of
one frame is the time between `detect_async` and its callback, both measured in this process with
`time.perf_counter_ns`. A frame that never gets a result is counted as DROPPED, and a drop is a
signal about 30 Hz, not noise.

Usage: py -3.10 s1_bench.py --model hand_landmarker.task --seconds 30 [--csv <outside the repo>]
"""
import argparse
import statistics
import sys
import time

import cv2
import mediapipe as mp

BaseOptions = mp.tasks.BaseOptions
HandLandmarker = mp.tasks.vision.HandLandmarker
HandLandmarkerOptions = mp.tasks.vision.HandLandmarkerOptions
VisionRunningMode = mp.tasks.vision.RunningMode

WIDTH, HEIGHT = 640, 480


def percentile(values, q):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int(q * (len(ordered) - 1)))]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", default="hand_landmarker.task")
    ap.add_argument("--seconds", type=int, default=30)
    ap.add_argument("--camera", type=int, default=0)
    ap.add_argument("--hands", type=int, default=2)
    ap.add_argument("--csv", default=None, help="per-result latencies; keep it OUTSIDE the repository")
    args = ap.parse_args()

    submitted = {}  # timestamp_ms -> perf_counter_ns at detect_async
    latencies_ms = []
    hands_per_result = []

    def on_result(result, output_image, timestamp_ms):
        t0 = submitted.pop(timestamp_ms, None)
        if t0 is None:
            return
        latencies_ms.append((time.perf_counter_ns() - t0) / 1e6)
        hands_per_result.append(len(result.hand_landmarks))

    options = HandLandmarkerOptions(
        base_options=BaseOptions(model_asset_path=args.model),
        running_mode=VisionRunningMode.LIVE_STREAM,
        num_hands=args.hands,
        result_callback=on_result,
    )

    cap = cv2.VideoCapture(args.camera)
    cap.set(cv2.CAP_PROP_FRAME_WIDTH, WIDTH)
    cap.set(cv2.CAP_PROP_FRAME_HEIGHT, HEIGHT)
    ok, frame = cap.read()
    if not ok:
        sys.exit("no frame from the camera")
    print(f"camera frame: {frame.shape[1]}x{frame.shape[0]}", file=sys.stderr)

    sent = 0
    last_ts = -1
    start = time.perf_counter_ns()
    with HandLandmarker.create_from_options(options) as landmarker:
        while time.perf_counter_ns() - start < args.seconds * 1_000_000_000:
            ok, frame = cap.read()
            if not ok:
                break
            rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb)
            # LIVE_STREAM wants strictly increasing timestamps: two frames in one ms collide.
            ts = max(last_ts + 1, (time.perf_counter_ns() - start) // 1_000_000)
            last_ts = ts
            submitted[ts] = time.perf_counter_ns()
            landmarker.detect_async(image, ts)
            sent += 1
        time.sleep(0.5)  # let the last callbacks land before the landmarker closes
    cap.release()
    elapsed_s = (time.perf_counter_ns() - start) / 1e9

    if not latencies_ms:
        sys.exit("no result came back: nothing to report")
    two_hands = sum(1 for n in hands_per_result if n >= 2)
    print(
        f"results {len(latencies_ms)}  sent {sent}  dropped {len(submitted)}  "
        f"results/s {len(latencies_ms) / elapsed_s:.1f}"
    )
    print(
        f"latency ms: median {statistics.median(latencies_ms):.2f}  "
        f"p95 {percentile(latencies_ms, 0.95):.2f}  max {max(latencies_ms):.2f}"
    )
    print(f"results with two hands: {two_hands} of {len(latencies_ms)}")
    if args.csv:
        with open(args.csv, "w", encoding="utf-8") as f:
            f.write("latency_ms,hands\n")
            for lat, n in zip(latencies_ms, hands_per_result):
                f.write(f"{lat:.3f},{n}\n")


if __name__ == "__main__":
    main()
