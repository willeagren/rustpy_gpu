import time
import rustpy_gpu as gp
import numpy as np

def time_numpy(arrs):
    times = []
    for (u, v) in arrs:
        t_start = time.perf_counter_ns()
        np.multiply(u, v)
        times.append((time.perf_counter_ns() - t_start) / 1e6)
    
    print(f'Average multiply time numpy: {sum(times) / len(times)} ms')

ll = [(np.random.uniform(-1.0, 1.0, size=(64, 3, 256, 256)).astype(np.float32), np.random.uniform(-1.0, 1.0, size=(64, 3, 256, 256)).astype(np.float32)) for _ in range(10)]
time_numpy(ll)

def time_rust(arrs):
    times = []
    for (u, v) in arrs:
        t_start = time.perf_counter_ns()
        gp.multiply(u, v)
        times.append((time.perf_counter_ns() - t_start) / 1e6)
    
    print(f'Average multiply time rust: {sum(times) / len(times)} ms')

time_rust(ll)