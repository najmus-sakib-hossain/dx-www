// Poorly formatted C++ code to demonstrate clang-format
#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

// Template function with poor formatting
template <typename T>
T max(T a, T b) {
  return (a > b) ? a : b;
}

// Class with inconsistent spacing
class Person {
 private:
  std::string name;
  int age;

 public:
  Person(std::string n, int a) : name(n), age(a) {}

  void print() const {
    std::cout << "Name: " << name << ", Age: " << age << std::endl;
  }

  std::string getName() const { return name; }
  int getAge() const { return age; }
  void setAge(int a) { age = a; }
};

// Function with complex formatting issues
void process_vector(std::vector<int>& vec) {
  std::sort(vec.begin(), vec.end());
  vec.erase(std::unique(vec.begin(), vec.end()), vec.end());

  for (auto it = vec.begin(); it != vec.end(); ++it) {
    std::cout << *it << " ";
  }
  std::cout << std::endl;
}

// Namespace with poor indentation
namespace math {
namespace advanced {
double power(double base, int exp) {
  double result = 1.0;
  for (int i = 0; i < exp; i++) {
    result *= base;
  }

  return result;
}
}  // namespace advanced
}  // namespace math

int main() {
  // Variable declarations with inconsistent spacing
  int x = 10, y = 20, z = 30;
  std::cout << "Max: " << max(x, y) << std::endl;

  // Vector operations
  std::vector<int> numbers = {5, 2, 8, 2, 9, 1, 5, 3};
  std::cout << "Original: ";
  for (auto n : numbers) std::cout << n << " ";
  std::cout << std::endl;

  process_vector(numbers);
  std::cout << "Processed: ";
  for (auto n : numbers) std::cout << n << " ";
  std::cout << std::endl;

  // Object creation
  Person alice("Alice", 30);
  Person bob("Bob", 25);
  alice.print();
  bob.print();

  // Lambda with poor formatting
  auto add = [](int a, int b) -> int { return a + b; };
  std::cout << "Sum: " << add(5, 10) << std::endl;

  // Namespace function call
  std::cout << "Power: " << math::advanced::power(2.0, 10) << std::endl;

  return 0;
}
