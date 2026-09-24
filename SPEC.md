# Bucketlist File Format

Version 1.0

A `.bucketlist` file is a UTF-8 encoded plain-text file containing
a list of bucket-list items.

Each line represents one item.

## Items

An incomplete item consists of arbitrary text:

    Ride a hot-air balloon

A completed item is indicated by the `,x` suffix:

    Ride a hot-air balloon,x

## Example

    Try floating
    Ride a hot-air balloon
    Visit Luxembourg,x
    Learn how to sail

## Rules

- Files SHOULD use the `.bucketlist` extension.
- Files MUST be UTF-8 encoded.
- Each non-empty line represents one item.
- An item ending in `,x` is considered completed.
- All other items are considered incomplete.

