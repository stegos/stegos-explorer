# Stegos blockchain explorer

Unofficial Blockchain explorer for [Stegos](stegos.com).

## Architecture
The architecture of project can be represented in ASCII-image:
```lang-none
                     +------------+        +------------+
                  +--+ Fetcher    +--------+ Stegos     |
   +----------+   |  |  Mainnet   |        |  Mainnet   |
   | Postgres +---+  +------------+        +------------+
   |  Database|
   |          +---+
   +----+-----+   |  +------------+        +------------+
        |         +--+ Fetcher    +--------+ Stegos     |
   +----+-----+      |  Testnet   |        |  Testnet   |
   |  API     |      +------------+        +------------+
   +----+-----+
        |
 - - - -|- - - - - - - - - - - - - - - - - - - - - - -
        |
   +----+------+
   | Vue.JS    |
   +-----------+

```
Component list:
- **Stegos Mainnet/ Stegos Testnet** - is running node, that report information about processed blocks to fetcher.
- **Fetcher** - is util, that collect information from stegos node, and save it into relation database, sources can be found at **/explorer_backend/src/bin/fetcher** .
- **Postgres Database** - Relation database, for storing preagregated stuff.
- **API** - is server, that represent GraphQL API for getting information from relation database, sources can be found at **/explorer_backend/src/bin/api** .
- **Vue.JS** - is a frontend that render all this information to end user, sources can be found at **/blockchain_explorer** .