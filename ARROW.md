# Apache Arrow enabling blazing fast speed 

> Apache Arrow is a development platform for in memory analytics. It contains a set of technologies that enable big data systems to process and move data fast.
It specifies a standardized language-independent columnar memory format for flat and hierarchical data, organized for efficient analytical operations on modern
hardware. 

The primary goal of [Apache Arrow](https://arrow.apache.org/) is to improve the speed of data analytics by creating a *standard column memory format* that 
any computer language can understand. Most of the time the different technologies (databases, programming languages, libraries...) implement their internal data
formats. What this leads to practically is that approximately 90\% of the processing time in a system is spent on serializing and deserializing the various data
formats. Something that Arrow enables is for the CPU to spend important cycles on analytical operations instead of tedious transformations between formats. 

## Example workflow 
A common data science workflow/pipeline could be the following:
1. [SQL](https://www.dremio.com/) engine
2. [Java Database Connectivity](https://www.javatpoint.com/java-jdbc)
3. Data in [JVM](https://medium.com/platform-engineer/understanding-java-memory-model-1d0863f6d973) memory
4. Copy from JVM to Python memory
5. Process the memory with [pandas](https://pandas.pydata.org/)
6. Apply ML/stat models on the pandas DataFrame

When performing this pipeline the most time consuming part will be the process of copying the data from the JVM memory to Python and then changing the orientation
of the table to columnar format from row format. Instead of spending precious CPU cycles copying the data around from one memory instance to another, what if we
could simply put the data in memory and pass around a reference/pointer to the data? That is precisely what Arrow does under the hood. By leveraging underlying C APIs
the Arrow record batches can be put directly in the *off-heap* memory instead of memory space that is managed by the JVM. The C data interface in Python will then
inform the [pyarrow](https://arrow.apache.org/docs/python/index.html) library of the pointer to the raw data buffers, allowing for in place itnerpretation of the data. Furthermore, since the Arrow tables are already in 
columnar format no table orientation have to be made when creating the pandas DataFrames.


The performance boost only grows with the number of rows and columns in your data, and can on large datasets reduce processing time from over 1 minute to 500 ms !! This is only an example
workflow and the implications that using Arrow have on its performance. Of course if you are not using Python the implications and results will differ, but if you
are chasing speed, then perhaps you should instead consider the native [Rust](https://docs.rs/arrow/latest/arrow/) implementation to the Arrow format.

