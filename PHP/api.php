<?php
// Database connection details
$host = 'localhost';
$port = '5432';
$dbname = 'api_test';
$user = 'test';
$password = 'test';

// Connect to the PostgreSQL database
$conn = pg_connect("host=$host port=$port dbname=$dbname user=$user password=$password");

// Check if the connection was successful
if (!$conn) {
    die("Connection failed: " . pg_last_error());
}

// Query the PostgreSQL table
$query = "SELECT * FROM test";
$result = pg_query($conn, $query);

// Check if the query was successful
if (!$result) {
    die("Query failed: " . pg_last_error());
}

// Fetch the data as an associative array
$data = pg_fetch_all($result);

// Close the database connection
pg_close($conn);

// Set the response headers
header('Content-Type: application/json');

// Convert the data to JSON and send the response
echo json_encode($data);
?>