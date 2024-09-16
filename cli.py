import argparse
import json
from converter import json_to_python, json_to_lua, json_to_javascript

def main():
    parser = argparse.ArgumentParser(description="Convert JSON to Python, Lua, and JavaScript formats.")
    parser.add_argument('input_file', type=str, help="Path to the input JSON file.")
    parser.add_argument('output_format', choices=['python', 'lua', 'javascript'], help="Format to convert JSON to.")

    args = parser.parse_args()

    with open(args.input_file, 'r') as file:
        json_data = json.load(file)


    if args.output_format == 'python':
        converted_data = json_to_python(json_data)
    elif args.output_format == 'lua':
        converted_data = json_to_lua(json_data)
    elif args.output_format == 'javascript':
        converted_data = json_to_javascript(json_data)


    print(converted_data)

if __name__ == "__main__":
    main()
