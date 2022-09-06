import pyarrow as pa
import rustarrow as ra
import numpy as np

np_arr = np.random.uniform(-1.0, 1.0, size=(512, 3, 256, 256))
py_arr = pa.Tensor.from_numpy(np_arr)
ru_arr = ra.get_arrow_array(py_arr)
