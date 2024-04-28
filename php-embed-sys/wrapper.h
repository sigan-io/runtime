// PHP

#include <php.h>
#include <php_ini.h>
#include <php_main.h>
#include <php_variables.h>

// SAPI

#include <SAPI.h>
#include <sapi/embed/php_embed.h>

// Zend

#include <zend.h>
#include <zend_API.h>
#include <zend_string.h>

// Zend Functions

zend_string *sigan_zend_string_init(const char *str, size_t len)
{
    return zend_string_init_fast(str, len);
}

void sigan_zend_string_release(zend_string *str)
{
    return zend_string_release(str);
}


// TODO: Setup custom SAPI module.

// TODO: Try `php_execute_script` and `php_register_variable` with name "_SERVER[variable_name]".