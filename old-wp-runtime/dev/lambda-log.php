<?php declare(strict_types=1);

use Psr\Log\LogLevel;

$lambda_log = new \Bref\Logger\StderrLogger( LogLevel::DEBUG );

/**
 * Logs a message at the specified level.
 */
function lambda_log( mixed $message, array $context = array(), string $level = LogLevel::INFO ) {
	global $lambda_log;

	if ( is_string( $message ) ) {
		$lambda_log->$level( $message, $context );
	} else {
		$lambda_log->$level( '{context}', array( 'context' => $message ) );
	}
}

/**
 * Logs a message at the DEBUG level.
 */
function lambda_debug( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::DEBUG );
}

/**
 * Logs a message at the INFO level.
 */
function lambda_info( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::INFO );
}

/**
 * Logs a message at the NOTICE level.
 */
function lambda_notice( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::NOTICE );
}

/**
 * Logs a message at the WARNING level.
 */
function lambda_warning( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::WARNING );
}

/**
 * Logs a message at the ERROR level.
 */
function lambda_error( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::ERROR );
}

/**
 * Logs a message at the CRITICAL level.
 */
function lambda_critical( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::CRITICAL );
}

/**
 * Logs a message at the ALERT level.
 */
function lambda_alert( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::ALERT );
}

/**
 * Logs a message at the EMERGENCY level.
 */
function lambda_emergency( mixed $message, array $context = array() ) {
	lambda_log( $message, $context, LogLevel::EMERGENCY );
}

/**
 * Logs function's call info.
 */
function lambda_inspect( $result = null ) {
	$backtrace = debug_backtrace( 0, 2 );
	$last      = $backtrace[1];

	lambda_debug(
		'{info}',
		array(
			'info' => array(
				'function' => $last['function'],
				'args'     => $last['args'],
				'result'   => boolval( $result ),
				'file'     => $last['file'],
				'line'     => $last['line'],
			),
		)
	);
}
