DROP TABLE IF EXISTS Users;
DROP TABLE IF EXISTS Device;
DROP TABLE IF EXISTS Process;

CREATE TABLE Users (
    userid serial,
    username varchar(255) NOT NULL,
    hashpw varchar(255) NOT NULL,
    firstname varchar(255) NOT NULL,
    lastname varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    PRIMARY KEY(userid)
);

CREATE TABLE Device (
    device_id serial,
    userid serial NOT NULL,
    machine_name varchar(255) NOT NULL,
    active_memory varchar(12) NOT NULL,
    total_memory varchar(12) NOT NULL,
    cpu_usage varchar(9) NOT NULL,
    disk_space_used varchar(12) NOT NULL,
    total_disk_space varchar(12) NOT NULL,
    PRIMARY KEY(device_id),
    FOREIGN KEY(userid) REFERENCES User(userid)
);

CREATE TABLE Process (
    process_id serial,
    device_id serial NOT NULL,
    process_name varchar(255) NOT NULL,
    num_threads varchar(8) NOT NULL,
    resident_memory varchar(14) NOT NULL,
    proc_virtual_memory varchar(8) NOT NULL,
    proc_cpu_usage varchar(8) NOT NULL,
    cpu_time varchar(11) NOT NULL,
    PRIMARY KEY(process_id),
    FOREIGN KEY(device_id) REFERENCES Device(device_id)
);
