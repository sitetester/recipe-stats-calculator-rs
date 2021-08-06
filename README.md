It launches separate thread to parse data from given JSON file.   
Data from JSON file is read in chunks (rather than loading whole file in memory).

Parsed object is passed to a `receiver` through `channel`.

Expected output is shown on `stdout`


_Important notes_

1. Property value `"delivery"` always has the following format: "{weekday} {h}AM - {h}PM", i.e. "Monday 9AM - 5PM"


Functional Requirements
------

1. Count the number of unique recipe names.
2. Count the number of occurrences for each unique recipe name (alphabetically ordered by recipe name).
3. Find the postcode with most delivered recipes.
4. Count the number of deliveries to postcode `10120` that lie within the delivery time between `10AM` and `3PM`, examples _(`12AM` denotes midnight)_:
    - `NO` - `9AM - 2PM`
    - `YES` - `10AM - 2PM`
5. List the recipe names (alphabetically ordered) that contain in their name one of the following words:
    - Potato
    - Veggie
    - Mushroom