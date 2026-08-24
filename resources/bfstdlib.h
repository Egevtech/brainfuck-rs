#include <stdio.h>
#include <stdlib.h>

typedef struct vector {
  size_t len;
  size_t current;
  int *data;
} vec_t;

vec_t *vector_init() {
  vec_t *vec = (vec_t *)malloc(sizeof(vec_t));

  vec->data = (int *)malloc(sizeof(int));
  vec->len = 0;
  vec->current = 0;

  return vec;
}

void vector_free(vec_t *vec) {
  free(vec->data);
  free(vec);
}

void vector_realloc(vec_t *vec, int value) {
  vec->data = (int *)realloc(vec->data, value);
  if (vec->data == NULL) {
    perror("realloc");
    exit(EXIT_FAILURE);
  }
}

void vector_add(vec_t *vec, int value) { vec->data[vec->current] += value; }

void vector_move(vec_t *vec, int delta) {
  if (vec->len < vec->current + delta) {
    size_t d = (vec->current + delta) - vec->len;
    vector_realloc(vec, d);
  }

  vec->current += delta;
}

void vector_debug(vec_t *vec) {
    printf("%d", vec->data[vec->current]);
}

void vector_print(vec_t *vec) {
    printf("%c", vec->data[vec->current]);
}

int vector_get(vec_t *vec) {
  return vec->data[vec->current];
}

extern void bf_entry(vec_t *vec);

int main(void) {

  vec_t *vec = vector_init();

  bf_entry(vec);

  vector_free(vec);

  putc('\n', stdout);

  return 0;
}
