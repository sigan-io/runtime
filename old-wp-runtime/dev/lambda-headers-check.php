<?php declare(strict_types = 1);

lambda_debug( 'Server:', array( 'server' => $_SERVER ) );

$message_ok     = 'Headers: OK';
$message_failed = 'Headers: FAILED';

$_SERVER['HTTP_HOST'] = isset( $_SERVER['HTTP_X_FORWARDED_HOST'] ) ? $_SERVER['HTTP_X_FORWARDED_HOST'] : $_SERVER['HTTP_HOST'];

$protocol = isset( $_SERVER['HTTP_X_FORWARDED_PROTO'] );
$host     = isset( $_SERVER['HTTP_HOST'] );

if ( $protocol && $host ) {
	lambda_debug( $message_ok );
} else {
	lambda_critical(
		$message_failed . "\n"
		. '- Protocol: {protocol}' . "\n"
		. '- Host: {host}' . "\n",
		array(
			'protocol' => $protocol ? 'OK' : 'MISS',
			'host'     => $host ? 'OK' : 'MISS',
		)
	);

	http_response_code( 500 );
	die( $message_failed );
}
