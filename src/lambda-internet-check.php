<?php

$message_ok     = 'Internet Access: OK';
$message_failed = 'Internet Access: FAILED';

$opts = array(
	'http' => array(
		'method'  => 'GET',
		'timeout' => 10,  // Timeout in seconds
	),
);

$context  = stream_context_create( $opts );
$response = file_get_contents( 'https://example.com', false, $context );

if ( false !== $response ) {
	lambda_debug( $message_ok );
} else {
	lambda_critical( $message_failed );

	http_response_code( 500 );
	die( $message_failed );
}
