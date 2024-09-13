#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Brush Brush;

typedef struct Color Color;

typedef struct StringBuffer {
  char *buffer;
  size_t len;
} StringBuffer;

/**
 * ------
 * Clone the structure
 * ------
 */
struct Brush *brush_clone(const struct Brush *s);

/**
 * ------
 * Print a debug of the struct to stdout
 * ------
 */
void brush_debug(const struct Brush *s);

/**
 * ------
 * Construct a new model
 * ------
 */
struct Brush *brush_default(void);

/**
 * ------
 * Gets the current value
 * ------
 */
int32_t brush_get_color(const struct Brush *source, struct Color *c_value);

/**
 * An identifier for the brush
 * ------
 * Gets the current value
 * ------
 */
int32_t brush_get_name(const struct Brush *source, struct StringBuffer c_value);

/**
 * ------
 * Gets the current value
 * ------
 */
int32_t brush_get_weight(const struct Brush *source, uint8_t *c_value);

/**
 * ------
 * Replaces the current value with the provided value
 * ------
 */
int32_t brush_with_color(struct Brush *source, const struct Color *value);

/**
 * An identifier for the brush
 * ------
 * Replaces the current value with the provided value
 * ------
 */
int32_t brush_with_name(struct Brush *source, const char *value);

/**
 * ------
 * Replaces the current value with the provided value
 * ------
 */
int32_t brush_with_weight(struct Brush *source, uint8_t value);

/**
 * ------
 * Convert the enum into a new variant type
 * ------
 */
int32_t color_as_blue(struct Color *res);

/**
 * ------
 * Convert the enum into a new variant type
 * ------
 */
int32_t color_as_green(struct Color *res);

/**
 * ------
 * Convert the enum into a new variant type
 * ------
 */
int32_t color_as_red(struct Color *res);

/**
 * ------
 * Clone the enum value
 * ------
 */
struct Color *color_clone(const struct Color *s);

/**
 * ------
 * Print a debug string of the enum to stdout
 * ------
 */
void color_debug(const struct Color *s);

/**
 * ------
 * Construct a new blank enum
 * ------
 */
struct Color *color_default(void);

/**
 * ------
 * Construct the enum as a specific variant
 * ------
 */
struct Color *color_from_blue(void);

/**
 * ------
 * Construct the enum as a specific variant
 * ------
 */
struct Color *color_from_green(void);

/**
 * ------
 * Construct the enum as a specific variant
 * ------
 */
struct Color *color_from_red(void);
