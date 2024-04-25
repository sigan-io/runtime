<?php declare(strict_types = 1);

define( 'WP_ROOT', $_SERVER['WP_ROOT'] ?? '/mnt/wordpress' );
define( 'VENDOR_ROOT', $_SERVER['VENDOR_ROOT'] ?? '/var/task/vendor' );

require_once VENDOR_ROOT . '/autoload.php';
require_once __DIR__ . '/lambda-log.php';
require_once __DIR__ . '/lambda-errors.php';
require_once __DIR__ . '/lambda-statics.php';
require_once __DIR__ . '/lambda-route.php';

$local_path = lambda_route();

// If the requested path is not a PHP script, serve it as a static file.
if ( ! str_ends_with( $local_path, '.php' ) ) {
	lambda_serve_static( $local_path );
}

require_once $local_path;
