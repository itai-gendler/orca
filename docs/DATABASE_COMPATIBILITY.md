# Database compatibility

Orca upgrades old libraries on startup. A successful migration must preserve
playlists, settings, and metadata, and it must be transactional. Test upgrades
with disposable copies of older databases; never test against a user's library.

`PRAGMA user_version` is the minimum schema version a reader must support. The
released native app supports version 2. Compatible additions, such as the
`liked_songs` table, retain version 2 because that app can safely ignore them.
Each addition is recorded in `orca_schema_migrations` so it runs once. A later
change that removes or changes data used by a version 2 reader must raise
`user_version` and make a consistent SQLite backup before migrating. Older
builds must refuse to open that newer database; rollback requires restoring
the matching backup, not editing the version number.

Before release, test both directions with disposable profiles: upgrade a copy
of the previous release's library and check that user data survives; for a
compatible addition, launch the previous release against the upgraded copy.
For a breaking migration, verify the previous release refuses the upgraded
copy and can open the backup after restoration.

Version 3 was briefly used by unreleased Liked Songs builds for an additive
table. It is reserved for that preview schema and must not be reused for a
different migration. Current builds recognize its Liked Songs table and repair
its reader version to 2 without deleting the table or likes. Unknown newer
versions are rejected before WAL mode or migrations can modify the database.

For development, pass `--data-dir` with a disposable profile. A development
build using the default profile shares it with the installed release and can
upgrade that release's database.
