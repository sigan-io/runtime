<?php
// phpcs:disable WordPress.DB.RestrictedFunctions

// Initialize a mysqli instance
$conn = mysqli_init();

// Set a connection timeout of 10 seconds
mysqli_options( $conn, MYSQLI_OPT_CONNECT_TIMEOUT, 10 );
mysqli_options( $conn, MYSQLI_OPT_READ_TIMEOUT, 10 );

$message_ok     = 'Database Access: OK';
$message_failed = 'Database Access: FAILED';

try {
	// Connect to the database
	$connected = mysqli_real_connect( $conn, $_ENV['DB_HOST'], $_ENV['DB_USER'], $_ENV['DB_PASSWORD'], $_ENV['DB_NAME'] );

	if ( ! $connected ) {
		lambda_critical( $message_failed );

		http_response_code( 500 );
		die( $message_failed );
	}
} catch ( \Throwable $e ) {
	lambda_critical( $message_failed );

	http_response_code( 500 );
	die( $message_failed );
}

lambda_debug( $message_ok );

// Close connection
mysqli_close( $conn );
