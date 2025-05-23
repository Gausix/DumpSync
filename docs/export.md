# DumpSync Command: Export

To create a database dump, you can use the following command:

```bash
dumpsync export
```

For connectting to a server, read the [Connecting to a Server](connection.md) guide.

### Command Breakdown

- **dumpsync**: This is the main command to invoke the DumpSync tool.
- **export**: This subcommand initiates the export process to create a dump of the specified database.

### Options

- **-i**: (Optional) Sets the interval (in seconds) for the dump process. In this example, the interval is set to 3600 seconds (1 hour). You can adjust this value based on your requirements.
- **-f**: (Optional) Indicates the file path where the dump will be saved. Replace `/path/to/` with the desired directory path on your system.
- **--encrypt**: (Optional) Encrypts the dump file using AES-256 encryption. This option requires a password to encrypt and decrypt the dump file.
- **--once**: (Optional) Exports the database dump only once without creating a recurring schedule.
- **--retain**: (Optional) The retainimum number of backups to retain for the dump. If the number of dumps exceeds this limit, the scheduler will be terminated.
- **--pdf**: (Optional) Generates a PDF report of the dump process with your settings and tables dumped.

### Example

```bash
dumpsync export
```

### Exporting only once time

To export a database dump only once without creating a recurring schedule, you can use the `--once` option:

```bash
dumpsync export --once
```

This command will create a dump of the specified database and then exit without creating a recurring schedule.

### Setting the Retainimum Number of Backups

To set the retainimum number of backups to retain for the dump, you can use the `--retain` option:

```bash
dumpsync export --retain 5
```

This command will create a dump of the specified database and retain a retainimum of 5 backups. If the number of backups exceeds this limit, the scheduler will be terminated.

### Encrypt Dumps

To create an encrypted dump file, you can add the `--encrypt` option to the command:

```bash
dumpsync export --encrypt
```

The encryption process use AES-256 encryption and will prompt you to enter a password for the encryption and decryption of the dump file.

### Generate a PDF Report

To generate a PDF report of the dump process, you can use the `--pdf` option:

```bash
dumpsync export --pdf
```

This command will create a PDF report of the dump process with your settings and tables dumped.

### Notes

- The export process will create a dump file of the specified database at the specified interval.
- If you choose to encrypt the dump file, you will need to provide a password during the encryption process.
- Ensure that the specified path for the dump exists and that you have the necessary permissions to write to that directory.
- Adjust the interval according to your backup strategy to ensure that you have up-to-date dumps without overwhelming your database resources.
