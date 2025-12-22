# Sample Python file to demonstrate formatting and linting

from typing import Dict, Any


def hello_world(name: str, age: int) -> None:
    """Print a greeting message.

    Args:
        name: The name of the person
        age: The age of the person
    """
    print(f"Hello, {name}! You are {age} years old.")


class Person:
    """A simple Person class."""

    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    def greet(self):
        """Greet the person."""
        print(f"Hello, my name is {self.name} and I am {self.age} years old.")


# Some poorly formatted code to test the formatter
def poorly_formatted(x: int, y: int, z: int):
    """This function has bad formatting."""
    result = x + y + z
    if result > 10:
        print("Result is greater than 10")
    else:
        print("Result is 10 or less")
    return result


# Test list comprehension
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
squares = [x**2 for x in numbers if x % 2 == 0]

# Test dictionary
data: Dict[str, Any] = {
    "name": "John",
    "age": 30,
    "city": "New York",
    "hobbies": ["reading", "gaming", "coding"],
}

# Long line that should trigger linting warning
very_long_variable_name = "This is a very long string that goes on and on and should probably be broken into multiple lines for better readability and maintainability"


if __name__ == "__main__":
    hello_world("Alice", 25)
    person = Person("Bob", 30)
    person.greet()
    result = poorly_formatted(1, 2, 3)
    print(f"Result: {result}")
    print(f"Squares: {squares}")
    print(f"Data: {data}")
