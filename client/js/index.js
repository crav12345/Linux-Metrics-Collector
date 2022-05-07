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
        	                // Ignore uuid column.
        	                if (k != "uuid" && k != "proc_kernel_mode_time" && k != "proc_user_mode_time") {
	        	                columns.push(k);
                         		
                         		switch (k) {
						case 'proc_id':
							k = "PID";
							break;
						case 'proc_name':
							k = "Process Name"
							break;
						case 'num_threads':
							k = "Threads"
							break;
						case 'proc_mem':
							k = "Memory"
							break;
						case 'proc_cpu':
							k = "CPU"
							break;
						case 'proc_bytes_read':
							k = "Read"
							break;
						case 'proc_bytes_written':
							k = "Written"
							break;
						case 'proc_disk_usage':
							k = "Disk"
							break;
						case 'proc_bytes_received':
							k = "Received"
							break;
						case 'proc_bytes_transmitted':
							k = "Transmitted"
							break;
						case 'proc_net_usage':
							k = "Network"
							break;
						default:
							console.log(`Couldn't find ${k}.`);
					}
                         		
        		                // Creating the header
	        	                header.append($('<th/>').html(k));
	        	        }
        	        }
                }
	}
             
        // Appending the header to the table
        $(selector).append(header);
        return columns;
}
