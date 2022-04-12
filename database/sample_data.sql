INSERT INTO Users(userid, username, hashpw, firstname, lastname, email)
VALUES (1, "connorfitz", "hashedpw", "Connor", "Fitzpatrick", "connorfitz@gmail.com");

INSERT INTO Users(userid, username, hashpw, firstname, lastname, email)
VALUES (2, "mikedurso111", "hashedpw", "Michael", "Durso", "mikedurso@yahoo.com");

INSERT INTO Users(userid, username, hashpw, firstname, lastname, email)
VALUES (3, "chrisrav000", "hashedpw", "Christopher", "Ravosa", "cravosa@aol.com");


INSERT INTO Device(device_id, userid, machine_name, active_memory, total_memory, cpu_usage, disk_space_used, total_disk_space)
VALUES (1, 1, "Connor's Linux VM", "2.11 GB", "4.00 GB", "34.23%", "101 GB", "1000 GB");

INSERT INTO Device(device_id, userid, machine_name, active_memory, total_memory, cpu_usage, disk_space_used, total_disk_space)
VALUES (2, 1, "Connor's Work Laptop", "4.55 GB", "8.00 GB", "10.00%", "48.88 GB", "250 GB");

INSERT INTO Device(device_id, userid, machine_name, active_memory, total_memory, cpu_usage, disk_space_used, total_disk_space)
VALUES (3, 2, "Mike's Laptop", "5.69 GB", "8.00 GB", "45.55%", "300 GB", "500 GB");

INSERT INTO Device(device_id, userid, machine_name, active_memory, total_memory, cpu_usage, disk_space_used, total_disk_space)
VALUES (4, 3, "Chris's PC", "4.91 GB", "16.00 GB", "65.11%", "500 GB", "1200 GB");


INSERT INTO Process(process_id, device_id, process_name, num_threads, resident_memory, proc_virtual_memory, proc_cpu_usage, cpu_time)
VALUES(1, 1, "Java", "4", "2.01 GB", "3.50 GB", "10.34%", "13:01");

INSERT INTO Process(process_id, device_id, process_name, num_threads, resident_memory, proc_virtual_memory, proc_cpu_usage, cpu_time)
VALUES(2, 1, "Intellij", "3", "1.13 GB", "1.40 GB", "5.98%", "10:44:01");

INSERT INTO Process(process_id, device_id, process_name, num_threads, resident_memory, proc_virtual_memory, proc_cpu_usage, cpu_time)
VALUES(3, 3, "Google Chrome", "2", "0.56 MB", "256 MB", "0.99%", "03:01");

INSERT INTO Process(process_id, device_id, process_name, num_threads, resident_memory, proc_virtual_memory, proc_cpu_usage, cpu_time)
VALUES(4, 4, "Discord", "3", "17 MB", "300 MB", "0.58%", "09:01");