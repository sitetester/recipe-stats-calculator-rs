It launches separate thread to parse data from given JSON file.   
Data from JSON file is read in chunks (rather than loading whole file in memory).

Parsed object is passed to a `receiver` through `channel`.

Expected output is shown on `stdout`