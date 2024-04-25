// PHP

#include <php.h>
#include <php_ini.h>
#include <php_main.h>

// Zend

#include <zend.h>
#include <zend_API.h>
#include <zend_string.h>

// Embed SAPI

#include <sapi/embed/php_embed.h>

// Make private functions available to Rust

zend_string *sigan_zend_string_init(const char *str, size_t len, bool persistent)
{
    return zend_string_init(str, len, persistent);
}

zend_string *sigan_zend_string_init_fast(const char *str, size_t len)
{
    return zend_string_init_fast(str, len);
}