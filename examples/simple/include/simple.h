#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Brush Brush;

typedef struct Color Color;

struct Brush *brush_clone(const struct Brush *s);

void brush_debug(const struct Brush *s);

struct Brush *brush_default(void);

/**
 * Replaces the current value with the provided value
 */
int32_t brush_get_color(const struct Brush *source, struct Color *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t brush_get_weight(const struct Brush *source, uint8_t *c_value);

/**
 * Replaces the current value with the provided value
 */
int32_t brush_with_color(struct Brush *source, const struct Color *value);

/**
 * Replaces the current value with the provided value
 */
int32_t brush_with_weight(struct Brush *source, uint8_t value);

int32_t color_as_blue(struct Color *res);

int32_t color_as_green(struct Color *res);

int32_t color_as_red(struct Color *res);

struct Color *color_clone(const struct Color *s);

void color_debug(const struct Color *s);

struct Color *color_default(void);

struct Color *color_from_blue(void);

struct Color *color_from_green(void);

struct Color *color_from_red(void);
