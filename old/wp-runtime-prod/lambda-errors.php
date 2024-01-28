<?php declare(strict_types=1);

class NotImplemented extends Exception {
	public function __construct( $message = 'This function is not yet implemented.', $code = 0, Exception $previous = null ) {
		$trace = debug_backtrace( 0, 2 );
		$last  = $trace[1];

		if ( $last['function'] ) {
			$message = $last['function'] . '() is not yet implemented.';
		}

		parent::__construct( $message, $code, $previous );
	}
}

class NotTested extends Error {
	public function __construct( $message = 'This function has not been yet tested.', $code = 0, Exception $previous = null ) {
		$trace = debug_backtrace( 0, 2 );
		$last  = $trace[1];

		if ( $last['function'] ) {
			$message = $last['function'] . '() has not been yet tested.';
		}

		parent::__construct( $message, $code, $previous );
	}
}

set_error_handler(
	function ( $error_number, $error_string, $error_file, $error_line ) {
		$exception = new \ErrorException( $error_string, $error_number, 0, $error_file, $error_line );
		$error     = array(
			'code'    => $error_number,
			'message' => $error_string,
			'file'    => $error_file,
			'line'    => $error_line,
			'trace'   => array_slice( $exception->getTrace(), 1 ),
		);

		lambda_error( '{error}', array( 'error' => $error ) );
	},
	E_ALL
);

set_exception_handler(
	function ( Throwable $exception ) {
		$exception = array(
			'code'    => $exception->getCode(),
			'message' => $exception->getMessage(),
			'file'    => $exception->getFile(),
			'line'    => $exception->getLine(),
			'trace'   => array_slice( $exception->getTrace(), 1 ),
		);

		lambda_critical( '{exception}', array( 'exception' => $exception ) );

		http_response_code( 500 );
		header( 'Content-Type: application/json' );
		die( json_encode( $exception, JSON_PRETTY_PRINT ) );
	}
);
