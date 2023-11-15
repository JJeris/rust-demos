import sys

# Check if the required argument is provided
if len(sys.argv) != 3:
    print("Usage: your_script.py -- <variable_to_print>")
    # sys.exit(1)

# Retrieve the variable to print from the command line arguments
variable_to_print = sys.argv[5]

# Print the variable
print(f"The variable passed from Rust is: {variable_to_print}")