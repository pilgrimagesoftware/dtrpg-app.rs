## ADDED Requirements

### Requirement: A dispatched download MUST fetch and write real file bytes
`LibraryService::download_item` MUST resolve a download URL for the requested item and write the fetched bytes to the destination path, so that `Downloaded` status only reflects a file that actually exists on disk.

#### Scenario: Successful transfer
- **WHEN** a download is dispatched for an item and the fetch completes without error
- **THEN** a file exists at the item's resolved on-disk path containing the fetched bytes

### Requirement: An in-progress transfer MUST NOT leave a partial file at the final path
The transfer MUST write to a temporary `.part` path and only place the file at its final destination after the full transfer succeeds.

#### Scenario: Transfer fails partway through
- **WHEN** a download's network fetch fails after only some bytes have been received
- **THEN** no file exists at the final destination path; at most a `.part` file remains

#### Scenario: Transfer completes successfully
- **WHEN** a download's fetch completes in full
- **THEN** the final destination path contains the complete file and no `.part` file remains

### Requirement: Cancelling an in-progress transfer MUST remove any partial data
When a download is cancelled while its transfer is in progress, no partial or final file MUST remain at the destination path.

#### Scenario: Cancelling mid-transfer
- **WHEN** the user cancels a download while bytes are still being fetched
- **THEN** the transfer stops, any `.part` file is deleted, and no file exists at the final destination path
