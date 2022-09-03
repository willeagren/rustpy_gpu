import rustpy_gpu as gp
import numpy as np
import pyarrow as pa

a = np.ones((1, 2))
b = np.zeros((1, 2))
pa_a = pa.array(a)
pa_b = pa.array(b)
rust_result = gp.add(a, b)
print(f"Result from rust binary: {rust_result}")
