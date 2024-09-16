# test.py

import json

# Sample JSON data
data = {
    "description": "My cool app!",
    "name": "NiCl",
    "version": "0.0.1"
}

# Convert the Python dictionary to a JSON string and print it
print(json.dumps(data, indent=4))
