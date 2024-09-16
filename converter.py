import json

def json_to_python(json_obj):
    """Convert JSON object to Python dictionary"""
    return json.dumps(json_obj, indent=4)

def json_to_lua(json_obj):
    """Convert JSON object to Lua table"""
    def to_lua(value, indent=0):
        spaces = ' ' * indent
        lua_str = ""
        
        if isinstance(value, dict):
            lua_str += "{\n"
            for key, val in value.items():
                lua_key = f'"{key}"' if isinstance(key, str) else str(key)
                lua_str += f'{spaces}  {lua_key} = {to_lua(val, indent + 2)},\n'
            lua_str = lua_str.rstrip(',\n') + '\n'  # Remove the trailing comma and newline
            lua_str += f"{spaces}}}"
        elif isinstance(value, list):
            lua_str += "{\n"
            for item in value:
                lua_str += f'{spaces}  {to_lua(item, indent + 2)},\n'
            lua_str = lua_str.rstrip(',\n') + '\n'  # Remove the trailing comma and newline
            lua_str += f"{spaces}}}"
        elif isinstance(value, str):
            lua_str += f'"{value}"'
        else:
            lua_str += str(value)
        
        return lua_str
    
    return to_lua(json_obj)

def json_to_javascript(json_obj):
    """Convert JSON object to JavaScript object"""
    return json.dumps(json_obj, indent=4)
