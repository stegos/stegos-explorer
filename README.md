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


