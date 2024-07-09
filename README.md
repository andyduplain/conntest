# conntest

A simple program to test internet connectivity.

# Network Access

When run from `cron` on macOS, the program is not allowed any network access, so you will need to run it via `sandbox-exec`:

```crontab
*/15 * * * * sandbox-exec -f source/conntest/sandbox-profile source/conntest/target/release/conntest https://github.com
```

The program will output to the project root directory, and it assumes it's being run from `<proj-root>/target/release`.
