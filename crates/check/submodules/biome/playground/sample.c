// Poorly formatted C code to demonstrate clang-format
#include <stdio.h>
#include <stdlib.h>

int add(int a, int b) { return a + b; }

void print_array(int* arr, int size) {
  for (int i = 0; i < size; i++) {
    printf("%d ", arr[i]);
  }
  printf("\n");
}

int main(void) {
  int x = 10, y = 20;
  int result = add(x, y);
  printf("Sum: %d\n", result);

  int numbers[] = {1, 2, 3, 4, 5};
  int size = sizeof(numbers) / sizeof(numbers[0]);
  print_array(numbers, size);

  // Memory allocation with poor formatting
  int* dynamic_array = (int*)malloc(5 * sizeof(int));
  if (dynamic_array == NULL) {
    fprintf(stderr, "Memory allocation failed\n");
    return 1;
  }

  for (int i = 0; i < 5; i++) {
    dynamic_array[i] = i * 2;
  }

  print_array(dynamic_array, 5);
  free(dynamic_array);

  return 0;
}
