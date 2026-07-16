# rust-resource-network-monitor Specification

## Purpose
TBD - created by archiving change implement-catalog-maintenance-behavior. Update Purpose after archive.
## Requirements
### Requirement: Network monitor distinguishes general and endpoint-specific connectivity
The app SHALL provide a network-monitor module that distinguishes general network
inaccessibility from unavailability of a specific remote endpoint, callable from catalog sync,
download, cover-cache, and avatar-cache code paths.

#### Scenario: General network is unavailable
- **WHEN** the device has no network connectivity at all
- **THEN** the network monitor reports general network unavailability

#### Scenario: General network is available but a specific endpoint is unreachable
- **WHEN** the device has network connectivity but the DriveThruRPG API endpoint cannot be
  reached
- **THEN** the network monitor reports that endpoint as unreachable while still reporting
  general connectivity as available

### Requirement: Endpoint-specific processes query the monitor before requesting
A process that requires access to a specific remote endpoint SHALL query the network monitor
for that endpoint's reachability before making a request, and SHALL NOT make the request if
the monitor reports the endpoint unreachable.

#### Scenario: Endpoint is reachable
- **WHEN** catalog sync, download, or image-cache code queries the network monitor about the
  DriveThruRPG API endpoint and it reports reachable
- **THEN** the calling code proceeds with the request

#### Scenario: Endpoint is unreachable
- **WHEN** catalog sync, download, or image-cache code queries the network monitor about the
  DriveThruRPG API endpoint and it reports unreachable
- **THEN** the calling code does not make the request

### Requirement: General-connectivity processes query the monitor before requesting
A process that requires only general network access SHALL query the network monitor for
general connectivity before proceeding, and SHALL NOT proceed if the monitor reports general
network unavailability.

#### Scenario: General network is available
- **WHEN** a process queries the network monitor for general connectivity and it reports
  available
- **THEN** the process proceeds

#### Scenario: General network is unavailable
- **WHEN** a process queries the network monitor for general connectivity and it reports
  unavailable
- **THEN** the process stops rather than proceeding

### Requirement: Network monitor caches recent check results to bound check frequency
The network monitor SHALL cache the result of a reachability check for a short interval so
that a burst of calling requests does not each trigger a new check.

#### Scenario: Repeated queries within the cache interval reuse the cached result
- **WHEN** the network monitor is queried more than once for the same target within its cache
  interval
- **THEN** only the first query performs a live reachability check; subsequent queries within
  the interval return the cached result
