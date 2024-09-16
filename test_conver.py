# test_conversion.py

import json
from converter import json_to_python, json_to_lua, json_to_javascript

# Sample JSON data
json_data = {
    "description": "My cool app!",
    "name": "NiCl",
    "version": "0.0.1"
}

# Convert JSON data to different formats
python_code = json_to_python(json_data)
lua_code = json_to_lua(json_data)
javascript_code = json_to_javascript(json_data)

# Print the converted formats
print("Python Code:\n")
print(python_code)
print("\nLua Code:\n")
print(lua_code)
print("\nJavaScript Code:\n")
print(javascript_code)
