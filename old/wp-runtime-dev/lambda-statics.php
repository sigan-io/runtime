<?php declare(strict_types=1);

function lambda_serve_static( string $local_path ) {
	$bucket_name   = $_SERVER['WP_BUCKET'];
	$bucket_domain = isset( $_SERVER['IS_LOCAL'] )
		? 'localhost:9000' . '/' . $bucket_name
		: $bucket_name . '.s3.amazonaws.com';
	$bucket_path   = str_replace( WP_ROOT, '', $local_path );
	$bucket_url    = $_SERVER['HTTP_X_FORWARDED_PROTO'] . '://' . $bucket_domain . $bucket_path;

	lambda_info(
		'Serving: {bucket_url}',
		array(
			'bucket_url' => $bucket_url,
		)
	);

	header( 'Status: 302 Found' );
	header( 'Location: ' . $bucket_url );
	exit;
}
