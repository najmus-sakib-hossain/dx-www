// Poorly formatted header file
#ifndef SAMPLE_H
#define SAMPLE_H

// Math operations
int add(int a, int b);
int subtract(int a, int b);
int multiply(int a, int b);
double divide(double a, double b);

// Array utilities
void print_array(int* arr, int size);
int find_max(int* arr, int size);
int find_min(int* arr, int size);

// String utilities
char* str_duplicate(const char* src);
int str_length(const char* str);

// Structure definitions
struct Point {
  int x;
  int y;
};

struct Rectangle {
  struct Point top_left;
  struct Point bottom_right;
};

// Function prototypes for structure operations
double calculate_distance(struct Point p1, struct Point p2);
double calculate_area(struct Rectangle rect);

#endif  // SAMPLE_H
