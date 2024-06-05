const express = require('express');
const { Pool } = require('pg');

const app = express();

// PostgreSQL database configuration
const dbConfig = {
    host: 'localhost',
    port: 5432,
    user: 'test',
    password: 'test',
    database: 'api_test',
};

// Create a PostgreSQL connection pool
const pool = new Pool(dbConfig);

// API endpoint to fetch data from the database
app.get('/api/data', (req, res) => {
    pool.query('SELECT * FROM test', (error, results) => {
        if (error) {
        console.error('Error executing query:', error);
        res.status(500).json({ error: 'Internal Server Error' });
        } else {
            res.json(results.rows);
        }
    });
});

// Start the server
const port = 3000;
app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});