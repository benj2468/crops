#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct Brush Brush;

typedef struct Color Color;

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
 * Free the memory allocated by an opque type's pointer
 *
 * # Safety
 *
 * This function requires that the value passed in be properly aligned by Box/Rust. This function will free the memory allocated at the pointer.
 * ------
 */
void brush_free(struct Brush *s);

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
int32_t brush_get_name(const struct Brush *source, char *c_value);

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
int32_t color_as_other(struct Color *res, const char *value);

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
 * Free the memory allocated by an opque type's pointer
 *
 * # Safety
 *
 * This function requires that the value passed in be properly aligned by Box/Rust. This function will free the memory allocated at the pointer.
 * ------
 */
void color_free(struct Color *s);
