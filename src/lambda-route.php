<?php

/**
 * Routes the URL paths to the file that will handle them.
 */
function lambda_route() {
	$method     = $_SERVER['REQUEST_METHOD'];
	$uri        = $_SERVER['REQUEST_URI'];
	$parsed_uri = parse_url( $uri );
	$url_path   = $parsed_uri['path'] ?? '';
	$local_path = WP_ROOT . rtrim( $url_path, '/' );

	lambda_info(
		'Request: {method} {uri}',
		array(
			'method' => $method,
			'uri'    => $uri,
		)
	);

	// Check if the path is not a forbidden file.
	$forbidden_files = '#^/(?:wp-content|wp-includes)/.*\.php$#';
	if ( preg_match( $forbidden_files, $local_path ) ) {
		header( 'Status: 403 Forbidden' );
		exit;
	}

	// Check if the path is not a protected file.
	$private_files = '#\.(?:crt|ini|htaccess|json|scss)$#';
	if ( preg_match( $private_files, $local_path ) ) {
		header( 'Status: 403 Forbidden' );
		exit;
	}

	if ( is_dir( $local_path ) ) {
		return $local_path . '/index.php';
	}

	// TODO: Support for uploads. Currently because new files are non existent
	// the code below will not work, and directing the request to index.php
	// instead of redirecting to the bucket.

	if ( is_file( $local_path ) ) {
		return $local_path;
	}

	// Covers the case when a path is a file but it doesn't exist yet.
	if ( strpos( basename( $local_path ), '.' ) !== false ) {
		return $local_path;
	}

	return WP_ROOT . '/index.php';
}
