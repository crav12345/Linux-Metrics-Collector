// TODO: Compress these into one function and make the api a variable name.

function getAllMetrics() {
	fetch('/api/metrics', {
		method: 'GET',
		headers: {
			'content-type': 'application/json; charset = UTF-8',
			'cache-control': 'no-cache' // <- finish removing cache and format tables.
		}
	})
	.then(response => response.json())
	.then(data => constructTable(data));
}

function getMemoryUsage() {
	fetch('/api/metrics/memory', {
		method: 'GET',
		headers: {
			'content-type': 'application/json; charset = UTF-8',
			'cache-control': 'no-cache' // <- finish removing cache and format tables.
		}
	})
	.then(response => response.json())
	.then(data => constructTable(data));
}

function getDiskUsage() {
	fetch('/api/metrics/disk', {
		method: 'GET',
		headers: {
			'content-type': 'application/json; charset = UTF-8',
			'cache-control': 'no-cache' // <- finish removing cache and format tables.
		}
	})
	.then(response => response.json())
	.then(data => constructTable(data));
}

function getCPUUsage() {
	fetch('/api/metrics/cpu', {
		method: 'GET',
		headers: {
			'content-type': 'application/json; charset = UTF-8',
			'cache-control': 'no-cache' // <- finish removing cache and format tables.
		}
	})
	.then(response => response.json())
	.then(data => constructTable(data));
}

function getNetworkUsage() {
	fetch('/api/metrics/network', {
		method: 'GET',
		headers: {
			'content-type': 'application/json; charset = UTF-8',
			'cache-control': 'no-cache' // <- finish removing cache and format tables.
		}
	})
	.then(response => response.json())
	.then(data => constructTable(data));
}

function constructTable(data) {
	// Remove all rows in the table before changing it.
	$("#table tr").remove(); 
	
	// Getting the all column names
        var cols = colHeaders(data, '#table'); 
  
        // Traversing the JSON data
        for (var i = 0; i < data.length; i++) {
	        var row = $('<tr/>');  
                for (var colIndex = 0; colIndex < cols.length; colIndex++) {
	                var val = data[i][cols[colIndex]];
                     
	                // If there is any key, which is matching
        	        // with the column name
                	if (val == null) val = ""; 
                        row.append($('<td/>').html(val));
                }
                 
                // Adding each row to the table
	        $('#table').append(row);
        }
}

function colHeaders(data, selector) {
	var columns = [];
        var header = $('<tr/>');
             
        for (var i = 0; i < data.length; i++) {
        	var row = data[i];
                 
                for (var k in row) {
        	        if ($.inArray(k, columns) == -1) {
        	                columns.push(k);
                         
        	                // Creating the header
        	                header.append($('<th/>').html(k));
        	        }
                }
	}
             
        // Appending the header to the table
        $(selector).append(header);
        return columns;
}
